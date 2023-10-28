pub mod demo_engine;

use demo_engine::{demo_triangle, info, menu, menu_choice, MENU_CHOICE};

use std::env;
use std::sync::Arc;
use std::thread;

use crossterm::terminal::enable_raw_mode;

use billy_engine::eventkeyboard::eventkeyboard;
use billy_engine::event::{subscribe, publish};
use billy_engine::engine::{create_default_engine, Point};
use billy_engine::ui::{Boxe, BoxeElement, FormeGraphique};
use billy_engine::audio;

static mut GAME_RUN: bool = true;

fn game_end() {
	unsafe {
		GAME_RUN = false;
	}
}
fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
        eprintln!("Usage: {} <chemin_du_repertoire_de_l_application>", args[0]);
        std::process::exit(1);
    }

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
		choice_demo.set_callback_by_ellement(0,|| {
			menu_choice(1);
		});
		choice_demo.set_callback_by_ellement(1,|| {
			menu_choice(2);
		});
		choice_demo.set_callback_by_ellement(2, game_end);
	}

	subscribe("ESC", game_end);

	bijour.set_title("Bijour".to_string());
	bijour.set_size(20, 10);
	bijour.write_text(0, true, &mut coucou);

	audio::test();

	let game = thread::spawn({
		let engine = Arc::clone(&engine);
		let choice_demo = Arc::clone(&choice_demo);
		move || {
			let choice_demo = Arc::clone(&choice_demo);
			let engine = Arc::clone(&engine);
			unsafe {
				while GAME_RUN {
					let choice_demo = Arc::clone(&choice_demo);
					let engine = Arc::clone(&engine);
					match MENU_CHOICE {
						0 => menu(engine, choice_demo, bijour.clone()),
						1 => demo_triangle(engine),
						2 => info(engine),
						_ => println!("no match")
					}
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

