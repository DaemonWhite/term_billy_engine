pub mod eventkeyboard {
	use std::{io, time::Duration};
	use lazy_static::lazy_static;
	use crossterm::event::{Event,read, poll, KeyCode, KeyEvent};
	use crate::event::{create, publish, subscribe};

	const KEYS_UP: &str = "UP";
	const KEYS_DOWN: &str = "DOWN";
	const KEYS_LEFT: &str = "LEFT";
	const KEYS_RIGHT: &str = "RIGHT";
	const KEYS_ENTER: &str = "ENTER";
	const KEYS_ESC: &str = "ESC";
	const KEYS_FOCUS: &str = "FOCUS";

	lazy_static! {
		static ref REGISTERY_KEYS: Vec<&'static str> = vec![
			KEYS_UP,
			KEYS_DOWN,
			KEYS_LEFT,
			KEYS_RIGHT,
			KEYS_ENTER,
			KEYS_ESC,
			KEYS_FOCUS
		];
	}

	static mut LISTEN_KEYBOARD: bool= true;

	pub fn end_keyboard() {
		unsafe {
			LISTEN_KEYBOARD = false;
		}
	}

	pub fn start_keyboard() {
		unsafe {
			LISTEN_KEYBOARD = true;
		}
	}

	pub fn init_event_keyboard() {
		let rk = &REGISTERY_KEYS;
		for _name in rk.iter() {
			create(_name);
		}
	}

	fn registery_keys(event: KeyEvent) {
		let event = event.code;
		subscribe("END_ENGINE", end_keyboard);
		match event {
			KeyCode::Enter => publish(KEYS_ENTER),
			KeyCode::Up => publish(KEYS_UP),
			KeyCode::Down => publish(KEYS_DOWN),
			KeyCode::Right => publish(KEYS_RIGHT),
			KeyCode::Left => publish(KEYS_LEFT),
			KeyCode::Esc => publish(KEYS_ESC),
			_ => eprintln!("No Keys registery\r")
		}
	}

	pub fn listen_keys() -> io::Result<()> {
		const REFRESH_KEYS: u64 = 500;
		unsafe {
			while LISTEN_KEYBOARD {
				if poll(Duration::from_millis(REFRESH_KEYS))? {
					match read()? {
						Event::FocusGained => publish(KEYS_FOCUS),
						// Event::Key(event) => println!("{:?}\r", event),
						Event::Key(event) => registery_keys(event),
						Event::Resize(_w, _h) => break,
						_ => println!("no match\r"),
					}
				}
			}
		}
		Ok(())
	}

	pub fn print_events() -> io::Result<()> {
		loop {
			match read()? {
				Event::FocusGained => println!("Enter\r"),
				Event::Key(event) => println!("{:?}\r", event),
				Event::Resize(_w, _h) => break,
				_ => println!("no match\r"),
			}
		}
		Ok(())
	}
}