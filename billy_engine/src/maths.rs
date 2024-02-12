pub(crate) use min_max::{max, min};

use crate::engine::Point;

/// Maco min_point
/// * p Passe en mètres 1 poins ou plus
/// L'objectif étant de créer un point avec les deux coordonée les plus faible
///
/// ```rust
///	use billy_engine::engine::Point;
/// use min_max::min; // la macro min_point dépend de min_max
/// let p1 = Point::new(1,3);
/// let p2 = Point::new(4,9);
/// let p3 = Point::new(9,1);
///
/// let p_min = billy_engine::min_point!(p1, p2, p3);
///
/// println!("{:?}", p_min); // x =1 y =1
///```

#[macro_export]
macro_rules! min_point {
    ($p:expr, $($ps:expr),+) => {
    	Point::new(
    		min!($p.get_x(), min!( $($ps.get_x()), +)),
    		min!($p.get_y(), min!( $($ps.get_y()), +))
    	)
    };
}

/// Maco max_point
/// * p Passe en mètres 1 poins ou plus
/// L'objectif étant de créer un point avec les deux coordonée les plus forte
///
/// ```rust
///	use billy_engine::engine::Point;
/// use min_max::max; // la macro min_point dépend de min_max
/// let p1 = Point::new(1,3);
/// let p2 = Point::new(4,9);
/// let p3 = Point::new(9,1);
///
/// let p_min = billy_engine::max_point!(p1, p2, p3);
///
/// println!("{:?}", p_min); // x =9 y =9
///```

#[macro_export]
macro_rules! max_point {
    ($p:expr, $($ps:expr),+) => {
    	Point::new(
    		max!($p.get_x(), max!( $($ps.get_x()), +)),
    		max!($p.get_y(), max!( $($ps.get_y()), +))
    	)
    };
}

/// Methode qui permet de vérifier le bon comportement de la macro
pub fn teste_macro() {
    let p1 = Point::new(1, 5);
    let p2 = Point::new(5, 3);
    let p3 = Point::new(2, 8);

    println!("{:?}", p1);

    let t_min = min_point!(p1, p2, p3);
    let t_max = max_point!(p1, p2, p3);

    println!("n min : {:?}", min_point!(p1, p2, p3));

    println!(
        " -- list point -- \n{:?}\n{:?}\n{:?}\nResult : {:?}\n",
        p1, p2, p3, t_min
    );
    println!(
        " -- list point -- \n{:?}\n{:?}\n{:?}\nResult : {:?}",
        p1, p2, p3, t_max
    );
}

/// Constuire un triangle à partir de ces point
/// * 'position' Donne la position à analisée
/// * 'p1' Point de refèrence 1
/// * 'p2' Point de reference 2
pub fn eq_triangle(position: Point, p1: Point, p2: Point) -> i16 {
    let _eq: i16 = (p1.get_x() - position.get_x()) * (p2.get_y() - position.get_y())
        - (p1.get_y() - position.get_y()) * (p2.get_x() - position.get_x());
    _eq
}

pub(crate) use max_point;
pub(crate) use min_point;
