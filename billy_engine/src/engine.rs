
/// Module de base du billy engine

use crate::maths;

use std::io::{self, Write, Stdout};
#[doc(hidden)]
extern crate crossterm;
#[doc(hidden)]
use crossterm::{ ExecutableCommand, terminal, cursor};

///Gestion d'un tableau à deux dimesion
#[derive(Clone, Copy, Debug)]
pub struct Point {
	x: i16,
	y: i16
}

impl Point {
	/// Permet de gérer les point
	/// # Arguments
    /// * 'x' Passe en paramètre la position de la largeure
    /// * 'y' Passe en parapmètre la position de la hauteur
	pub fn new (x: i16,y: i16) -> Point {
    	Point {
            x: x,
            y: y
        }
    }

    pub fn get_x(&self) -> i16 {
        self.x
    }

    pub fn get_y(&self) -> i16 {
        self.y
    }
    /// Deplace un point de "position + nb" case
    /// # Arguments
    ///  * 'y' Valeur de déplacement
    /// ```rust
    /// use billy_engine::engine::Point;
    /// let mut p = Point::new(4,1); // x = 4, y =1
    /// p.move_y(5) // x = 4, y = 6
    /// ```
    pub fn move_y(&mut self, y: i16 ) {
        self.y += y;
    }

    /// Deplace un point de "position + nb" case
    /// # Arguments
    ///  * 'x' Valeur de déplacement
    /// ```rust
    /// use billy_engine::engine::Point;
    /// let mut p = Point::new(4,1); // x = 4, y = 1
    /// p.move_y(5) // x = 9, y = 1
    /// ```
    pub fn move_x(&mut self, x: i16 ) {
        self.x += x;
    }

    pub fn set_y(&mut self, y: i16) {
        self.y = y;
    }

    pub fn set_x(&mut self, x: i16) {
        self.x = x;
    }
}
/// Contient les informations de la fenètre
pub struct ScreenData {
	/// Largeur de l'écrans
    width: u16,
    /// Hauteur de l'écrans
    heigth: u16,
    /// Decalage de la hauteur peut êtres utiles dans certain terminale
    offset: u8
}

impl ScreenData {
    pub fn new() -> ScreenData {
        let offset: u8= 0;
		let (terminal_width, mut terminal_heigth) = terminal::size().unwrap();
        terminal_heigth -= offset as u16;
        ScreenData {
            width: terminal_width,
            heigth: terminal_heigth,
            offset: offset
        }
    }

    pub fn get_heigth(&self) -> u16 {
    	self.heigth - self.offset as u16
    }

    pub fn get_width(&self) -> u16 {
    	self.width
    }

    pub fn get_offset(&self) -> u8 {
    	self.offset
    }

    ///Permet de recalculer la taille de la fenètre
    pub fn refresh(&mut self) {
        let (terminal_width, terminal_heigth) = terminal::size().unwrap();
        self.width = terminal_width;
        self.heigth = terminal_heigth- self.offset as u16;
    }

    /// Retourne la taille de l'écrans
    ///
    /// # Return
    /// rend 'width' puis 'heigth'
    ///
    /// # Example
    /// ```rust
    /// use billy_engine::engine::ScreenData;
    ///
    /// let mut sd = ScreenData::new();
    /// let (width, heigth) = sd.size();
    /// // Ou pour récupérer une valeur
    /// let width = sd.size().0;
    /// ```
    pub fn size(&self) -> (u16, u16) {
        return (self.width, self.heigth - self.offset as u16);
    }
    /// Définie le offset celui-ci change l'auteur de l'écrans
    /// 'heigth - offset'
    pub fn set_offset(&mut self, offset: u8) {
        self.offset = offset;
    }
}
/// Permet de créer un triangle et de le modifier
#[derive(Debug,Clone, Copy)]
pub struct Triangle {
    position: [Point; 3],
    min_poistion: Point,
    max_position: Point
}

impl Triangle {
	/// Consturcteur à en paramètre
	/// * 'p1' donne le premier point
	/// * 'p2' oonne le deuxième point
	/// * 'p3' donne le troisième point
	///
	/// ```rust
	/// use billy_engine::engine::{Triangle, Point};
	///
	/// let t1 = Triangle::new(
	///		Point::new(1, 0),
	/// 	Point::new(5, 5),
	///     Point::new(5, 10)
	///	);

    pub fn new(p1:Point, p2:Point, p3:Point) -> Self {
        let min = maths::macro_min_point!(p1, p2, p3);
        let max = maths::macro_max_point!(p1, p2, p3);
        Triangle {
            position: [p1,p2,p3],
            min_poistion: min,
            max_position: max
        }
    }
    /// Permet de trouver les point les plus faible
    pub fn calculate_min_max_position(&mut self) {
    	self.min_poistion = maths::min_point( &self.position);
        self.max_position = maths::max_point( &self.position);
    }
	/// Deplace tout le triangle
    pub fn translate(&mut self, x:i16,y: i16) {
        for i in 0..3 {
            self.position[i].move_x(x);
            self.position[i].move_y(y);
        }
        self.calculate_min_max_position();
    }

	/// Deplace un point du triangle
    pub fn translate_point(&mut self, index: usize, x: i16, y: i16) {
        self.position[index].move_x(x);
        self.position[index].move_y(y);
    }

    pub fn get_point(&self, index: usize) -> Point {
        self.position[index]
    }

    pub fn get_min(&self) -> Point {
        self.min_poistion
    }

    pub fn get_max(&self) -> Point {
        self.max_position
    }
}

/// Base du moteur
pub struct BillyEngine {
	stdout: Stdout,
    sd: ScreenData,
    pixel_buffer: Vec<Vec<char>>,
}

impl BillyEngine {
    pub fn new() -> BillyEngine {
		let mut std = io::stdout();
		std.execute(cursor::Hide);
        let mut sd = ScreenData::new();
        sd.set_offset(1);

        let mut width: Vec<char> = Vec::new();
        width.resize(sd.width as usize, ' ');
        let mut pixel_buffer: Vec<Vec<char>> = Vec::new();
        pixel_buffer.resize(sd.heigth as usize - sd.offset as usize, width);
        BillyEngine {
        	stdout: std,
            sd: sd,
            pixel_buffer: pixel_buffer
        }
    }

    pub fn get_resolution(&self) -> (u16, u16) {
        self.sd.size()
    }

	/// Dessine la matrix
    pub fn draw(&mut self) {
    	self.stdout.execute(terminal::Clear(terminal::ClearType::All));
    	for h in 0..self.pixel_buffer.len() {
    		for w in 0..self.pixel_buffer[h].len() {
    			print!("{}", self.pixel_buffer[h][w]);
    		}
    	}
    	self.stdout.flush();
    }

	/// Veriffie la position
    pub fn verfif_position(&self, pixel: i16, max: i16) -> bool {
        let mut verif = false;
        const MIN: i16=0;
        if MIN <= pixel && pixel < max {
            verif = true;
        }
        return  verif;
    }

	/// Place un pixel dans la matrix
    pub fn put_pixel(&mut self, px: i16, py: i16, character: char) {
		self.sd.refresh();
        if  self.verfif_position(px, self.sd.width as i16)
            && self.verfif_position(py, self.sd.heigth as i16){
            self.pixel_buffer[py as usize][px as usize] = character;
        }
    }

	/// Place un texte
    pub fn put_texte(&mut self, texte: &str, position: Point) {
        const OFFSET: i16 = 1;
        let mut position_x = position.get_x();

        for chararcter in texte.chars() {
            if position_x >= 0 && position_x <= self.sd.width as i16 {
                self.put_pixel(
                    position_x,
                    position.get_y(),
					chararcter
                )
            }
			position_x += OFFSET;
        }
    }

	/// Place un triangle
    pub fn put_triangle(&mut self, triangle: &Triangle) {
        let ymin = isize::from(triangle.get_min().get_y());
        let ymax = isize::from(triangle.get_max().get_y());
        let xmin = isize::from(triangle.get_min().get_x());
        let xmax = isize::from(triangle.get_max().get_x());

        for y in ymin..ymax {
            if 0 <= y && y < self.sd.heigth as isize {
                for x in xmin..xmax  {
                    if 0 <= x && x < self.sd.width as isize {
                        let x = x as i16;
                        let y = y as i16;
                        let position = Point::new(x, y);
                        let w1 = maths::eq_triangle(position, triangle.get_point(2), triangle.get_point(0));
                        let w2 = maths::eq_triangle(position, triangle.get_point(0), triangle.get_point(1));
                        let w3 = maths::eq_triangle(position, triangle.get_point(1), triangle.get_point(2));
                        if w1 >= 0 && w2 >= 0 && w3 >= 0 || -w1 >= 0 && -w2 >= 0 && -w3 >= 0  {
                            self.put_pixel(x, y, '*')
                        }
                    }
                }
            }
		}
    }
    /// Remplace toute les valeurs de la Matrix
	pub fn clear(&mut self, character: char) {
		for h in 0..self.pixel_buffer.len() {
    		for w in 0..self.pixel_buffer[h].len() {
    			self.pixel_buffer[h][w] = character;
    		}
    	}
	}
}




