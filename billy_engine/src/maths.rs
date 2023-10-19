pub(crate) use min_max::{min, max};

use crate::engine::Point;

#[macro_export]
macro_rules! min_point {
    ($p:expr, $($ps:expr),+) => {
    	Point::new(
    		min!($p.get_x(), min!( $($ps.get_x()), +)),
    		min!($p.get_y(), min!( $($ps.get_y()), +))
    	)
    };
}


#[macro_export]
macro_rules! max_point {
    ($p:expr, $($ps:expr),+) => {
    	Point::new(
    		max!($p.get_x(), max!( $($ps.get_x()), +)),
    		max!($p.get_y(), max!( $($ps.get_y()), +))
    	)
    };
}

pub fn teste_macro() {
    let p1 = Point::new(1, 5);
    let p2 = Point::new(5, 3);
    let p3 = Point::new(2, 8);

    println!("{:?}", p1);

    let t_min = min_point!(p1,p2,p3);
    let t_max = max_point!(p1,p2,p3);

    println!("n min : {:?}", min_point!(p1, p2, p3));

    println!(" -- list point -- \n{:?}\n{:?}\n{:?}\nResult : {:?}\n", p1, p2, p3, t_min);
    println!(" -- list point -- \n{:?}\n{:?}\n{:?}\nResult : {:?}", p1, p2, p3, t_max);

}

pub fn eq_triangle(position: Point, p1: Point, p2: Point) -> i16 {
    let _eq: i16 =
        (p1.get_x() - position.get_x()) *
        (p2.get_y() - position.get_y() ) -
        (p1.get_y() - position.get_y()) *
        (p2.get_x() - position.get_x());
    _eq
}

pub(crate) use min_point;
pub(crate) use max_point;