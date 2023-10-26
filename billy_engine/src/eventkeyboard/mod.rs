pub mod eventkeyboard {
	use std::{io, time::Duration};
	use std::sync::{Arc, Mutex};
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

	pub trait ControllerUi {
		fn key_up(&mut self);
		fn key_down(&mut self);
		fn key_right(&mut self);
		fn key_left(&mut self);
		fn key_enter(&mut self);
		fn key_esc(&mut self);
		fn focus(&mut self);
	}

	pub fn link_event_keyboard<C>(c: Arc<Mutex<C>>)
	where
		C: ControllerUi + Send +'static,
	{
		let up = Arc::clone(&c);
		let down = Arc::clone(&c);
		let right = Arc::clone(&c);
		let left = Arc::clone(&c);
		let enter = Arc::clone(&c);
		let esc = Arc::clone(&c);
		let focus = Arc::clone(&c);

		subscribe(KEYS_UP,
			move || {
				let mut  c = up.lock().unwrap();
				c.key_up();
			}
		);

		subscribe(KEYS_DOWN,
			move || {
				let mut  c = down.lock().unwrap();
				c.key_down();
			}
		);

		subscribe(KEYS_LEFT,
			move || {
				let mut  c = left.lock().unwrap();
				c.key_left();
			}
		);

		subscribe(KEYS_RIGHT,
			move || {
				let mut  c = right.lock().unwrap();
				c.key_right();
			}
		);

				subscribe(KEYS_ENTER,
			move || {
				let mut  c = enter.lock().unwrap();
				c.key_enter();
			}
		);

		subscribe(KEYS_ESC,
			move || {
				let mut  c = focus.lock().unwrap();
				c.key_esc();
			}
		);

		subscribe(KEYS_FOCUS,
			move || {
				let mut  c = esc.lock().unwrap();
				c.focus();
			}
		);
	}
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
		subscribe("GAME_END", end_keyboard);
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
						Event::Key(event) => registery_keys(event),
						_ => println!("no match\r"),
					}
				}
			}
		}
		Ok(())
	}
}