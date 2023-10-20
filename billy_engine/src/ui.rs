use crate::engine::Point;

type Callback = fn();

fn error_callback() {
	eprint!("Callback no defined");
}

pub trait FormeGraphique {
	fn set_title(&mut self, title: String);
	fn set_size(&mut self, width: u16, heigth: u16);
	fn set_position(&mut self, p: Point);
	fn get_position(&self) -> Point;
	fn get_size(&self) -> [u16; 2];
	fn get_image(&self) -> &Vec<Vec<char>>;
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
	title: String,
	image: Vec<Vec<char>>,
}

impl Boxe {
	pub fn new(title: String) -> Self {
		let size: [u16; 2] = [10, 10];
		let mut w: Vec<char> = Vec::new();
		w.resize(size[0] as usize, ' ');
		let mut h: Vec<Vec<char>> = Vec::new();
		h.resize(size[0] as usize, w);
		Boxe {
			size: size,
			position: Point::new(1,1),
			title,
			image: h,
		}
	}
	fn write_title(&mut self) {
		let width = self.size[0] as usize-2;
		let heigth = self.size[1] as usize;
		let title_len = {
			if self.title.len() > width {
				width
			} else {
				self.title.len()+2
			}
		};

		if heigth >= 3 && width > 4{
			for w in 2..title_len {
				self.image[1][w] = self.title.chars().nth(w-2).unwrap();
			}
		}
		self.draw_border_line(2);
	}

	fn draw_border_line(&mut self, heigth: usize) {
		const DEFAULT_CHAR: char = '=';
		for _i in 0..(self.size[0] as usize -1) {
			self.image[heigth][_i] = DEFAULT_CHAR;
		}
	}

	fn draw_border_column(&mut self, width: usize) {
		const DEFAULT_CHAR: char = '=';
		for _i in 0..self.size[1] as usize {
			self.image[_i][width] = DEFAULT_CHAR;
		}
	}

	fn calcuate_border(&mut self) {
		//top line
		self.draw_border_line(0);
		//left line
		self.draw_border_column(0);
		//rigth line
		self.draw_border_column(self.size[0] as usize -1);
		//botom Line
		self.draw_border_line(self.size[1] as usize -1)
	}
	pub fn calculate(&mut self) {
		let mut w: Vec<char> = Vec::new();
		self.image.clear();
		// Width Table
		w.resize(self.size[0] as usize, ' ');
		// Heigth table
		self.image.resize(self.size[1] as usize, w);
		self.write_title();
		self.calcuate_border();
	}
}

impl FormeGraphique for Boxe {
	fn set_title(&mut self, title: String) {
		self.title = title;
		self.write_title();
	}
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
	fn get_image(&self) ->  &Vec<Vec<char>> {
		&self.image
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
			boxe: Boxe::new("".to_string()),
			elements: Vec::new(),
			selector: 0
		}
	}
}

impl FormeGraphique for BoxeElement {
	fn set_title(&mut self, title: String) {
		self.boxe.set_title(title);
	}
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
	fn get_image(&self) ->  &Vec<Vec<char>> {
		&self.boxe.image
	}
}
