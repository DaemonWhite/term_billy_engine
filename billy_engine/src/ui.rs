use crate::engine::Point;
use crate::{DEFAULT_CHAR,DEFAULT_CHAR_SELECT, DEFAULT_CHAR_BORDER};
type Callback = fn();

fn error_callback() {
	eprint!("Callback no defined");
}

pub trait FormeGraphique {
	fn set_title(&mut self, title: String);
	fn set_size(&mut self, width: u16, height: u16);
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
	pub fn set_select(&mut self, select: bool) {
		if !self.is_select && select {
			self.title.insert(0, DEFAULT_CHAR);
			self.title.insert(1, DEFAULT_CHAR_SELECT);
		} else if self.is_select && !select {
			self.title.remove(0);
			self.title.remove(0);
		 }
		 self.is_select = select;
	}
	pub fn get_name(&self) -> &String {
		&self.title
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
		w.resize(size[0] as usize, DEFAULT_CHAR);
		let mut h: Vec<Vec<char>> = Vec::new();
		h.resize(size[0] as usize, w);
		Boxe {
			size: size,
			position: Point::new(1,1),
			title,
			image: h,
		}
	}

	pub fn write_text(&mut self, position: usize, back_to_line: bool, text: &String) {
		let width = self.size[0] as usize-2;
		let height = self.size[1] as usize -3;
		let h_pos = position +3;

		let text_len: usize = {
			if text.len() > width && !back_to_line {
				width
			} else {
				text.len()-1
			}
		};

		let h_len: usize = {
			if back_to_line {
				let mut e: usize = (text_len as f32 / width as f32).ceil() as usize;
				e+=1;
				if e > height {
					e =height;
				}
				e
			} else {
				1 as usize
			}
		};

		let mut cursor = 0;
		for h in 0..h_len {
			for w in 2..width  {
				self.image[h+h_pos][w] = text.chars().nth(cursor).unwrap();
				if cursor >= text_len {
					break;
				}
				cursor += 1;
			}
		}

	}

	fn write_title(&mut self, position: usize) {
		let width = self.size[0] as usize-2;
		let height = self.size[1] as usize;
		let title_len = {
			if self.title.len() > width {
				width
			} else {
				self.title.len()+1
			}
		};

		if height >= 3 && width > 4{
			for w in 2..title_len {
				self.image[position][w] = self.title.chars().nth(w-2).unwrap();
			}
		}
		self.draw_border_line(2);
	}

	fn draw_border_line(&mut self, height: usize) {
		const DEFAULT_CHAR: char = DEFAULT_CHAR_BORDER;
		for _i in 0..(self.size[0] as usize -1) {
			self.image[height][_i] = DEFAULT_CHAR;
		}
	}

	fn draw_border_column(&mut self, width: usize) {
		const DEFAULT_CHAR: char = DEFAULT_CHAR_BORDER;
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
		w.resize(self.size[0] as usize, DEFAULT_CHAR);
		// Heigth table
		self.image.resize(self.size[1] as usize, w);

		self.write_title(1);
		self.calcuate_border();
	}
}

impl FormeGraphique for Boxe {
	fn set_title(&mut self, title: String) {
		self.title = title;
		self.write_title(0);
	}
	fn set_size(&mut self, width: u16, height: u16) {
		self.size = [width, height];
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
	selector: usize,
	multi_select: bool,
	nb_element: usize
}

impl BoxeElement {
	pub fn new(title: String) -> Self {
		BoxeElement {
			boxe: Boxe::new(title),
			elements: Vec::new(),
			selector: 0,
			multi_select: false,
			nb_element: 0
		}
	}
	fn render_option(&mut self) {
		for _e in 0..self.nb_element {
			self.boxe.write_text(_e, false, &self.elements[_e].get_name());
		}
	}
	pub fn calculate(&mut self) {
		self.boxe.calculate();
		self.render_option();
	}
	pub fn unselect_all(&mut self) {
		for i in 0..self.nb_element {
			self.elements[i].set_select(false);
		}
		self.calculate();
	}
	pub fn select_olny(&mut self, index: usize) {
		self.unselect_all();
		self.elements[index].set_select(true);
		self.calculate();
	}

	pub fn select_all(&mut self) {
		for i in 0..self.nb_element {
			self.elements[i].set_select(true);
		}
		self.calculate();
	}

	pub fn select_elements(&mut self, index: usize) {
		if !self.multi_select {
			self.unselect_all();
		}
		self.elements[index].set_select(true);
		self.selector = index ;
		self.calculate();
	}

	pub fn unselect_element(&mut self, index: usize) {
		if !self.multi_select {
			self.unselect_all();
		}
		self.elements[index].set_select(false);
		self.selector = index ;
		self.calculate();
	}

	pub fn action(&self) {
		self.elements[self.selector].action();
	}
	pub fn add_element(&mut self, name: String) {
		let el = Element::new(self.nb_element as u16, name);
		self.elements.push(el);
		self.nb_element +=1;
		self.render_option();
	}
	pub fn set_callback_by_ellement(&mut self, index: usize, c: Callback) {
		self.elements[index].set_action(c);
	}
}

impl FormeGraphique for BoxeElement {
	fn set_title(&mut self, title: String) {
		self.boxe.set_title(title);
	}
	fn set_size(&mut self, width: u16, height: u16) {
		self.boxe.set_size(width, height);
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
