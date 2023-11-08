use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::thread::JoinHandle;

use rodio::{decoder::Decoder, OutputStream, Sink, OutputStreamHandle, source::Source, source::Buffered};
use rodio::cpal::traits::HostTrait;
use rodio::cpal::{Device, Host};

#[derive(Clone, Copy, Debug)]
pub enum ChanelType {
	Infinite,
	Simple,
	Single
}

pub struct Song {
	name: String,
	chanel: usize,
	path: String,
	preload: bool,
	decoder: Option<Buffered<Decoder<BufReader<File>>>>
}

// TODO Utiliser les HashMap
// TODO Intégrer l'utiliter des chanels
// TODO Gérer correctement les threads

impl Song {
	pub fn new(name: &str, chanel: usize, path: &str, preload: bool) -> Self {
		let mut song = Song {
			name: name.to_string(),
			chanel: chanel,
			path: path.to_string(),
			preload: preload,
			decoder: None
		};
		if song.is_preload() {
			song.load();
		}
		song
	}

	pub fn get_name(&self) -> &str {
		self.name.as_str()
	}

	pub fn get_chanel_id(&self) -> usize {
		self.chanel
	}

	pub fn get_song(&mut self) -> Option<Buffered<Decoder<BufReader<File>>>> {
		self.decoder.clone()
	}

	pub fn is_preload(&self) -> bool {
		self.preload
	}

	pub fn load(&mut self) {
		if self.decoder.is_none() {
		    let file = File::open(self.path.as_str()).expect("Failed to open file");
		    let reader = BufReader::new(file);
		    let decoder = Decoder::new(reader).expect("eee");
		    self.decoder = Some(decoder.buffered());
    	}
	}

	pub fn unload(&mut self) {
		self.decoder = None;
	}
}

#[derive(Debug)]
pub struct Chanel {
	id: usize,
	chanel_type: ChanelType,
	name: String,
	volume: f32,
	played: Option<JoinHandle<()>>
}

impl Chanel {
	pub fn new(id: usize, chanel_type: ChanelType, name: &str, volume: f32) -> Self {
		Chanel {
			id: id,
			chanel_type: chanel_type,
			name: name.to_string(),
			volume: volume,
			played: None
		}
	}
	pub fn get_id(&self) -> usize {
		self.id
	}

	pub fn get_chanel_type(&self) -> ChanelType {
		self.chanel_type
	}

	pub fn get_name(&self) -> &str {
		self.name.as_str()
	}

	pub fn get_volume(&self) -> f32 {
		self.volume
	}

	pub fn set_volume(&mut self, volume: f32) {
		self.volume = volume;
	}

	pub fn is_played(&self) -> bool {
		if let Some(played) = &self.played {
			played.is_finished()
		} else {
			true
		}
	}

	pub fn playe_chanel (&mut self, decoder: Buffered<Decoder<BufReader<File>>>, sink: Sink) {
		sink.append(decoder);
		self.played = Some(thread::spawn(move || {
			sink.play();
			sink.sleep_until_end();
		}));
	}
}

pub struct SongController {
	list_song: Vec<Song>,
	list_chanel: Vec<Chanel>,
	_host: Host,
	_device: Device,
	_stream: OutputStream,
	stream_handle: OutputStreamHandle,
	default_path: String
}

impl SongController {
	pub fn new(path: &str) -> Self {
		let host = rodio::cpal::default_host();
		let device = host.default_output_device().expect("No output device available");

		let (_stream, stream_handle) = OutputStream::try_from_device(&device).unwrap();
		SongController {
			default_path: path.to_string(),
			list_song: Vec::new(),
			list_chanel: Vec::new(),
			_host: host,
			_device: device,
			_stream: _stream,
			stream_handle: stream_handle
		}
	}

	pub fn get_chanel_index(&self, chanel_name: &str) -> f32 {
		let mut iterator: f32 = -1.0;
		let len = self.list_chanel.len();
		for i in 0..len {
			if self.list_chanel[i].get_name() == chanel_name {
				iterator = i as f32;
				break;
			}
		}
		iterator
	}

	pub fn get_chanel_index_by_id(&self, id: usize) -> f32 {
		let mut iterator: f32 = -1.0;
		let len = self.list_chanel.len();
		for i in 0..len {
			if self.list_chanel[i].get_id() == id {
				iterator = i as f32;
				break;
			}
		}
		iterator
	}

	pub fn get_chanel_id(&self, chanel_name: &str) -> usize {
		let chanel_index = self.get_chanel_index(chanel_name);
		self.list_chanel[chanel_index as usize].get_id()
	}

	fn get_song_index(&self, song_name: &str) -> f32 {
		let mut iterator: f32 = -1.0;
		let len = self.list_song.len();
		for i in 0..len {
			if self.list_song[i].get_name() == song_name {
				iterator = i as f32;
				break;
			}
		}
		iterator
	}

	pub fn creat_channel(&mut self, chanel_type: ChanelType, name: &str) {
		let len = self.list_chanel.len();
		let chanel = Chanel::new(len, chanel_type, name, 1.0);
		self.list_chanel.push(chanel);
	}

	pub fn creat_song(&mut self, name: &str, chanel_name: &str, path: &str, preload:bool) {
		let path = format!("{}/{}",self.default_path, path);
		println!("{}", path);
		const NO_VALID: f32 = -1.0;
		let chanel_index = self.get_chanel_index(chanel_name);
		if NO_VALID  < chanel_index {
			let song = Song::new(name, self.list_chanel[chanel_index as usize].get_id(), path.as_str(), preload);
			self.list_song.push(song);
		} else {
			eprintln!("chanel non existant")
		}
	}

	fn read_song(&mut self, song_index: usize, chanel_index: usize) {
		if let Some(decoder) =  self.list_song[song_index].get_song() {
			let sink = Sink::try_new(&self.stream_handle).unwrap();
			self.list_chanel[chanel_index].playe_chanel(decoder, sink);
		}
	}

	fn song_start(&mut self, song_index: usize) {
		let chanel_id = self.list_song[0].get_chanel_id();
		let chanel_index: usize = self.get_chanel_index_by_id(chanel_id) as usize;
		let chanel_type = self.list_chanel[chanel_index].get_chanel_type();
		if !self.list_song[song_index].is_preload() {
			self.list_song[song_index].load();
		}
		match chanel_type {
			ChanelType::Single => self.read_song(song_index, chanel_index),
			ChanelType::Infinite => println!("loop"),
			ChanelType::Simple => println!("Simple")
		}
	}

	pub fn played_song(&mut self, song_name: &str) {
		const NO_VALID: f32 = -1.0;
		let song_index = self.get_song_index(song_name);
		if song_index > NO_VALID {
			self.song_start(song_index as usize);
		}
	}
}