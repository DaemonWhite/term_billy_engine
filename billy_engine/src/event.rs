type Callback = fn();

use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
	static ref LE: Mutex<ListEvent> = Mutex::new(
		ListEvent::new()
	);
}

#[derive(Debug)]
struct Event {
	name: String,
	subscribed: Vec<Callback>
}

impl Event {
	pub fn new(name: String) -> Self {
		let v: Vec<Callback> = Vec::new();
		Event {
			name: name,
			subscribed: v
		}
	}
	pub fn get_name(&self) -> &String {
		&self.name
	}
	pub fn set_callback(&mut self, c: Callback) {
		self.subscribed.push(c);
	}
	pub fn call(&self) {
		for _e in 0..self.subscribed.len() {
			self.subscribed[_e]();
		}
	}
}


#[derive(Default, Debug)]
struct ListEvent {
	nb: usize,
	list_event: Vec<Event>
}

impl ListEvent {
	pub fn new () -> Self {
		let v: Vec<Event> = Vec::new();
	    ListEvent {
	    	nb: 0,
	    	list_event: v
	    }
	}
	pub fn add_event(&mut self, name: String) {
		let event = Event::new(name);
		self.nb += 1;
		self.list_event.push(event);
	}

	fn index_event_by_name(&self, name: String) -> isize {
		let mut index: isize = -1;
		for _i in 0..self.nb {
			if self.list_event[_i].get_name() == &name {
				index = _i as isize;
				break;
			}
		}
		return index;
	}

	pub fn subscribe(&mut self, name: String, c: Callback) {
		let index = self.index_event_by_name(name);
		if index != -1 {
			self.list_event[index as usize].set_callback(c);
		} else {
			eprint!("index inconue");
		}
	}

	pub fn call(&self, name: String) {
		let index = self.index_event_by_name(name);
		if index != -1 {
			self.list_event[index as usize].call();
		} else {
			eprint!("index inconue");
		}
	}
}

pub fn add_event(name: String) {
	let mut le = LE.lock().unwrap();
	le.add_event(name);
}

pub fn subscribed_event(name: String, c: Callback) {
	let mut le = LE.lock().unwrap();
	le.subscribe(name, c);
}

pub fn call(name: String) {
	let le = LE.lock().unwrap();
	le.call(name);
}

pub fn list_events() {
	let le = LE.lock().unwrap();
	println!("{:?}", le)
}

// static mut list_event: ListEvent = ListEvent::new();
