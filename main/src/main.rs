pub mod demo_engine;

use demo_engine::{demo_triangle, info};

static mut GAME_RUN: bool = true;

use crossterm::terminal::enable_raw_mode;
use billy_engine::eventkeyboard::eventkeyboard;
use billy_engine::event::{subscribe, publish};
use billy_engine::engine::{create_default_engine, Point};
use billy_engine::ui::{Boxe, BoxeElement, FormeGraphique};
use std::sync::Arc;
use std::time::Duration;
use std::thread;

fn game_end() {
	unsafe {
		GAME_RUN = false;
	}
}
fn main() {
	eventkeyboard::init_event_keyboard();
	let _ = enable_raw_mode();
	let keyboard = thread::spawn(|| {
		let _ = eventkeyboard::listen_keys();
	});
	let engine = create_default_engine();
	// list_events();
	let mut bijour = Boxe::new("Bonjour".to_string());
	let mut coucou: String = "Tu sais que les billy son les meilleure ?".to_string();
	let choice_demo = BoxeElement::new("Choisie".to_string());
	{
		let choice_demo = Arc::clone(&choice_demo);
		let mut choice_demo = choice_demo.lock().unwrap();
		choice_demo.set_title("choisie ta démo".to_string());
		choice_demo.set_position(Point::new(21, 1));
		choice_demo.set_size(20, 10);
		choice_demo.add_element("Triangle".to_string());
		choice_demo.add_element("Info".to_string());
		choice_demo.add_element("Fermer".to_string());
		choice_demo.select_elements(0);
		choice_demo.set_callback_by_ellement(0, demo_triangle);
		choice_demo.set_callback_by_ellement(1, info);
		choice_demo.set_callback_by_ellement(2, game_end);
	}

	subscribe("ESC", game_end);

	bijour.set_title("Bijour".to_string());
	bijour.set_size(20, 10);
	bijour.write_text(0, true, &mut coucou);

	let game = thread::spawn({
		let engine = Arc::clone(&engine);
		let choice_demo = Arc::clone(&choice_demo);
		move || {
			unsafe {
				while GAME_RUN {
					let _ = enable_raw_mode();
					thread::sleep(Duration::from_millis(100));
					let choice_demo = choice_demo.lock().unwrap();
					let mut engine = engine.lock().unwrap();
					engine.clear(' ');
					engine.put_object(choice_demo.clone());
					engine.put_object(bijour.clone());
					engine.draw();
				}
			}
			publish("GAME_END");
		}
	});
	game.join().unwrap();
	keyboard.join().unwrap();
	{
		let engine = Arc::clone(&engine);
		let engine = engine.lock().unwrap();
		engine.cleanup();
	}
	println!("Vous êtes beau\r");
	//publish("RESIZE");
}

