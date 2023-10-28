use std::fs::File;
use std::io::{BufReader, Seek};
use std::error::Error;
use std::sync::{Arc, Mutex};
use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle, Source};

pub struct Song {
	name: String,
	chanel: usize,
	path: String,
	preload: bool,
	decoder: Option<BufReader<File>>
}

struct MySource;



impl Song {
	pub fn new(name: &str, chanel: usize, path: &str, preload: bool) -> Self {
		let mut song = Song {
			name: name.to_string(),
			chanel: 0,
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

	pub fn get_song(&self) -> &BufReader<dyn Seek > {
		let mut arc_decoder: &BufReader<File>;

		match &self.decoder {
			Some(decoder) => arc_decoder = decoder,
			_ =>  {
				let buf: &BufReader<File> = BufReader::new(File::open(self.path.as_str()).unwrap());
				arc_decoder = &buf;
			}
		}


		arc_decoder
	}

	pub fn is_preload(&self) -> bool {
		self.preload
	}

	pub fn load(&mut self) {
		self.decoder =  Some(
					BufReader::new(File::open(self.path.as_str()).unwrap())
				);
	}

	pub fn unload(&mut self) {
		self.decoder = None;
	}
}

pub struct Chanel {
	id: usize,
	chanel_type: u8,
	name: String,
	volume: f32,
}

impl Chanel {
	pub fn new(id: usize, chanel_type: u8, name: &str, volume: f32) -> Self {
		Chanel {
			id: id,
			chanel_type: chanel_type,
			name: name.to_string(),
			volume: volume
		}
	}
	pub fn get_id(&self) -> usize {
		self.id
	}

	pub fn get_chanel_type(&self) -> u8 {
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
}

pub struct SongController {
	list_song: Vec<Song>,
	list_chanel: Vec<Chanel>,
	stream_handle: OutputStreamHandle
}

impl SongController {
	pub fn new() -> Self {
		let (_stream, stream_handle) = OutputStream::try_default().unwrap();
		SongController {
			list_song: Vec::new(),
			list_chanel: Vec::new(),
			stream_handle: stream_handle
		}
	}

	pub fn creat_channel(&mut self, chanel_type: u8, name: &str) {
		let len = self.list_chanel.len();
		let chanel = Chanel::new(len, chanel_type, name, 1.0);
		self.list_chanel.push(chanel);
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

	pub fn creat_song(&mut self, name: &str, chanel_name: &str, path: &str, preload:bool) {
		const NO_VALID: f32 = -1.0;
		let chanel_index = self.get_chanel_index(chanel_name);
		if NO_VALID  < chanel_index {
			let song = Song::new(name, self.list_chanel[chanel_index as usize].get_id(), path, preload);
			self.list_song.push(song);
		} else {
			eprintln!("chanel non existant")
		}
	}

	fn read_song(self, song_index: usize) {

		let mut sink = Sink::try_new(&self.stream_handle).unwrap();

		let decoder =  self.list_song[song_index].get_song();
		let decoder = Decoder::new(decoder);
		// sink.append(decoder);
		// sink.play();
		// sink.sleep_until_end();

		loop {
			if sink.empty() {
				break;
			}
		}
	}

	fn song_start(&self, song_index: usize) {
		let chanel_id = self.list_song[0].get_chanel_id();
		let chanel_index = self.get_chanel_index_by_id(chanel_id);
		let Chanel_type = self.list_chanel[chanel_index as usize].get_chanel_type();
		if self.list_song[song_index].is_preload() {

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


pub fn test() {
	// Get a output stream handle to the default physical sound device
	let (_stream, stream_handle) = OutputStream::try_default().unwrap();
	// Load a sound from a file, using a path relative to Cargo.toml
	let file = BufReader::new(File::open("examples/04 Flagcarrier.flac").unwrap());
	// Decode that sound file into a source
	let source = Decoder::new(file).unwrap();

	// Play the sound directly on the device
	//let _ = stream_handle.play_raw(source.convert_samples());

	let mut sink = Sink::try_new(&stream_handle).unwrap();
	sink.append(source);
	sink.play();
	sink.sleep_until_end();

	loop {
		if sink.empty() {
			break;
		}
	}

	// The sound plays in a separate audio thread,
	// so we need to keep the main thread alive while it's playing.
	std::thread::sleep(std::time::Duration::from_secs(5));

}