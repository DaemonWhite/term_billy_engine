
/// Module de base du billy engine
pub mod engine {
    use crate::maths::maths;
    extern crate crossterm;
    use crossterm::{terminal};

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
        /// let mut sd = ScreenData::new();
        /// let (width, heigth) = sd.size();
        /// // Ou pour récupérer une valeur
        /// let width = sd.size().0
        /// ```

        pub fn size(&self) -> usize {
            return usize::from( self.width * self.heigth);
        }
        /// Définie le offset celui-ci change l'auteur de l'écrans
        /// 'heigth - offset'
        pub fn set_offset(&mut self, offset: u8) {
            self.offset = offset;
        }
    }
    #[derive(Debug,Clone, Copy)]
    pub struct Triangle {
        position: [Point; 3],
        min_poistion: Point,
        max_position: Point
    }

    impl Triangle {
        pub fn new(p1:Point, p2:Point, p3:Point) -> Self {
            let min = maths::macro_min_point!(p1, p2, p3);
            let max = maths::macro_max_point!(p1, p2, p3);
            Triangle {
                position: [p1,p2,p3],
                min_poistion: min,
                max_position: max
            }
        }
        pub fn calculate_min_max_position(&mut self) {
             self.min_poistion = maths::min_point( &self.position);
             self.max_position = maths::max_point( &self.position);
        }

        pub fn translate(&mut self, x:i16,y: i16) {
            for i in 0..3 {
                self.position[i].move_x(x);
                self.position[i].move_y(y);
            }
            self.calculate_min_max_position();
        }

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

    pub struct BillyEngine {
        sd: ScreenData
    }

    impl BillyEngine {
        pub fn new() -> BillyEngine {
            let mut sd = ScreenData::new();
            sd.set_offset(1);
            BillyEngine {
                sd: ScreenData::new()
            }
        }

        pub fn get_size(&self) -> usize {
            return self.sd.size() as usize;
        }

        pub fn get_resolution(&self) -> (u16, u16) {
            (self.sd.width, self.sd.heigth)
        }

        pub fn draw(&mut self, pixel_buffer: &mut [char] ) {
            self.sd.refresh();
            for _e in 0..self.sd.size() {
                print!("{}", pixel_buffer[_e]);
            }
        }

        pub fn verfif_position(&self, pixel: i16, max: i16) -> bool {
            let mut verif = false;
            const MIN: i16=0;
            if MIN <= pixel && pixel < max {
                verif = true;
            }
            return  verif;
        }

        pub fn put_pixel(&mut self, px: i16, py: i16, character: char, pixel_buffer: &mut [char]) {
            self.sd.refresh();
            if  self.verfif_position(px, self.sd.width as i16)
                && self.verfif_position(py, self.sd.heigth as i16){
                pixel_buffer[usize::from(py as u16 * self.sd.width  + px as u16)] = character;
            }
        }

        pub fn put_texte(&mut self, texte: &str, position: Point,pixel_buffer: &mut [char]) {
            let mut offset: i16 = 0;
            for chararcter in texte.chars() {
                if (position.get_x() + offset) >= 0 {
                    self.put_pixel(
                        position.get_x() + offset,
                        position.get_y(),
                        chararcter,
                        pixel_buffer
                        )
                }
                offset += 1;
            }

        }

        pub fn put_triangle(&mut self, triangle: &Triangle, pixel_buffer: &mut [char]) {
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
                                self.put_pixel(x, y, '*', pixel_buffer)
                            }
                        }

                    }
                }
            }
        }

        pub fn clear(&mut self, character: char, pixel_buffer: &mut [char]) {
            for _e in 0..self.sd.size() {
                pixel_buffer[_e] = character;
            }
        }

    }
}




