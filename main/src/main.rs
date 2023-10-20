pub mod demo_engine;

use billy_engine::engine::BillyEngine;
use billy_engine::ui::{Boxe, FormeGraphique};
fn main() {
	let mut engine = BillyEngine::new();
	let mut choice_demo = Boxe::new("Bonjour".to_string());
	choice_demo.set_title("Bijour".to_string());
	choice_demo.set_size(11, 10);
	engine.clear(' ');
	engine.put_object(choice_demo);
	engine.draw();
}

