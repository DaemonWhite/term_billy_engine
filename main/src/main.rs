pub mod demo_engine;

use billy_engine::engine::BillyEngine;
use billy_engine::ui::{Boxe, FormeGraphique};
fn main() {
	let mut engine = BillyEngine::new();
	let mut choice_demo = Boxe::new();
	choice_demo.set_size(20, 30);
	engine.clear(' ');
	engine.put_object(choice_demo);
	engine.draw();
}

