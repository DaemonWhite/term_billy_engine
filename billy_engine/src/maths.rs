

use crate::engine::Point;

#[macro_export]
macro_rules! min_point {
    ( $( $point:expr ),* ) => {
		{
		    let mut min: Point = $($point;)+
		    $(
		    	if min.get_x() > $point.get_x() {
		            min.set_x($point.get_x());
		        }
		        if min.get_y() > $point.get_y() {
		            min.set_y($point.get_y());
		        }
		    )*
		    min
		}
    };
}

#[macro_export]
macro_rules! max_point {
( $( $point:expr ),* ) => {
    {
        let mut min: Point = $($point;)+
        $(
            if min.get_x() < $point.get_x() {
                min.set_x($point.get_x());
            }
            if min.get_y() < $point.get_y() {
                min.set_y($point.get_y());
            }
		)*
        min
    }};
}

pub fn teste_macro() {
    let p1 = Point::new(1, 9);
    let p2 = Point::new(5, 3);
    let p3 = Point::new(2, 8);

    println!("{:?}", p1);

    let t_min = min_point!(p1,p2,p3);
    let t_max = max_point!(p1,p2,p3);

    println!(" -- list point -- \n{:?}\n{:?}\n{:?}Result : \n{:?}\n", p1, p2, p3, t_min);
    println!(" -- list point -- \n{:?}\n{:?}\n{:?}\nResult : {:?}", p1, p2, p3, t_max);
}

pub fn min_point(point: &[Point]) -> Point {
    let mut min = point[0];
    let size = point.len();

    for _i in 1..size {
        if min.get_x() > point[_i].get_x() {
            min.set_x(point[_i].get_x());
        }
        if min.get_y() > point[_i].get_y() {
            min.set_y(point[_i].get_y());
        }
    }
    min
}

pub fn max_point(point: &[Point]) -> Point {
    let mut max = point[0];
    let size = point.len();

    for _i in 1..size {
        if max.get_x() < point[_i].get_x() {
            max.set_x(point[_i].get_x());
        }
        if max.get_y() < point[_i].get_y() {
            max.set_y(point[_i].get_y());
        }
    }
    max
}

pub fn eq_triangle(position: Point, p1: Point, p2: Point) -> i16 {
    let _eq: i16 =
        (p1.get_x() - position.get_x()) *
        (p2.get_y() - position.get_y() ) -
        (p1.get_y() - position.get_y()) *
        (p2.get_x() - position.get_x());
    _eq
}

pub(crate) use min_point as macro_min_point;
pub(crate) use max_point as macro_max_point;