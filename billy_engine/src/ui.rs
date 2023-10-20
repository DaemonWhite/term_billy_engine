use crate::engine::Point;

type Callback = fn();

fn error_callback() {
	eprint!("Callback no defined");
}

pub trait FormeGraphique {
	fn set_size(&mut self, width: u16, heigth: u16);
	fn set_position(&mut self, p: Point);
	fn get_position(&self) -> Point;
	fn get_size(&self) -> [u16; 2];
	fn get_pixel(&self) -> Vec<Vec<char>>;
}

#[derive(Clone)]
pub struct Element {
	id: u16,
	title: String,
	is_select: bool,
	action: Callback,
}

impl Element {
	pub fn new(id: u16, s: String) -> Self {
		Element {
			id: id,
			title: s,
			is_select: false,
			action: (error_callback)
		}
	}
	pub fn set_name(&mut self, title: String) {
		self.title = title;
	}
	pub fn set_action(&mut self, c: Callback) {
		self.action = c;
	}
	pub fn get_id(&self) -> u16 {
		self.id
	}
	pub fn is_selected(&self) -> bool {
		self.is_select
	}
	pub fn action(&self) {
		self.action;
	}
}

#[derive(Clone, Debug)]
pub struct Boxe {
	size: [u16; 2],
	position: Point,
	image: Vec<Vec<char>>,
}

impl Boxe {
	pub fn new() -> Self {
		let size: [u16; 2] = [10, 10];
		let mut w: Vec<char> = Vec::new();
		w.resize(size[0] as usize, ' ');
		let mut h: Vec<Vec<char>> = Vec::new();
		h.resize(size[0] as usize, w);
		Boxe {
			size: size,
			position: Point::new(1,1),
			image: h,
		}
	}
	fn calcuate_border(&mut self) {
		const DEFAULT_CHAR: char = '=';
		let width = self.size[0] as usize-1;
		let heigth = self.size[1] as usize-1;
		println!("{}x{} = {}x{}", width, heigth, self.image[0].len(), self.image.len());
		//top line
		for _i in 0..width {
			self.image[0][_i] = DEFAULT_CHAR;
		}
		//left line
		for _i in 0..heigth {
			self.image[_i][0] = DEFAULT_CHAR;
		}
		//rigth line
		for _i in 0..heigth {
			self.image[_i][width] = DEFAULT_CHAR;
		}
		// Bottom Line
		for _i in 0..(width+1) {
			self.image[heigth][_i] = DEFAULT_CHAR;
		}
	}
	pub fn calculate(&mut self) {
		let mut w: Vec<char> = Vec::new();
		self.image.clear();
		w.resize(self.size[0] as usize, ' ');
		println!("{}", w.len());
		self.image.resize(self.size[1] as usize, w);
		self.calcuate_border();
	}
}

impl FormeGraphique for Boxe {
	fn set_size(&mut self, width: u16, heigth: u16) {
		self.size = [width, heigth];
		self.calculate();
	}
	fn set_position(&mut self, p: Point) {
		self.position = p;
	}
	fn get_size(&self) -> [u16; 2] {
		self.size
	}
	fn get_position(&self) -> Point {
		self.position
	}
	fn get_pixel(&self) -> Vec<Vec<char>> {
		self.image.clone()
	}
}

#[derive(Clone)]
pub struct BoxeElement {
	boxe: Boxe,
	elements: Vec<Element>,
	selector: u16,
}

impl BoxeElement {
	pub fn new() -> Self {
		BoxeElement {
			boxe: Boxe::new(),
			elements: Vec::new(),
			selector: 0
		}
	}
}

impl FormeGraphique for BoxeElement {
	fn set_size(&mut self, width: u16, heigth: u16) {
		self.boxe.set_size(width, heigth);
	}
	fn set_position(&mut self, p: Point) {
		self.boxe.set_position(p);
	}
	fn get_size(&self) -> [u16; 2] {
		self.boxe.get_size()
	}
	fn get_position(&self) -> Point {
		self.boxe.get_position()
	}
	fn get_pixel(&self) -> Vec<Vec<char>> {
		self.boxe.image.clone()
	}
}
