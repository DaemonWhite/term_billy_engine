pub mod demo_engine;

use billy_engine::engine::{BillyEngine, Point};
use billy_engine::ui::{Boxe, BoxeElement, FormeGraphique};
fn main() {
	let mut engine = BillyEngine::new();
	let mut bijour = Boxe::new("Bonjour".to_string());
	let mut coucou: String = "Tu sais que les billy son les meilleure ?".to_string();
	println!("{}", coucou.len());
	let mut choice_demo = BoxeElement::new("Choisie".to_string());
	choice_demo.set_title("choisie ta démo".to_string());
	choice_demo.set_position(Point::new(21, 1));
	choice_demo.set_size(20, 10);
	choice_demo.add_element("Triangle".to_string());
	choice_demo.add_element("Autre".to_string());
	choice_demo.select_elements(0);

	bijour.set_title("Bijour".to_string());
	bijour.set_size(20, 10);
	bijour.write_text(0, true, &mut coucou);

	engine.clear(' ');
	engine.put_object(choice_demo);
	engine.put_object(bijour);
	engine.draw();
}

