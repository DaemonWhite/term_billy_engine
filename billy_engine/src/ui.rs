use crate::engine::Point;

type Callback = fn();

pub struct Element {
	id: u16,
	texte: String,
	is_select: bool,
	action: Callback,
}



pub struct Boxe {
	size: [u16; 2],
	position: Point,
	element: Option<Vec<Element>>,
	selector: u16
}

impl Boxe {
	fn new(&self) -> Self {
		Boxe {
			size: [10, 20],
			position: Point::new(1,1),
			element: None,
			selector: 0
		}
	}
}