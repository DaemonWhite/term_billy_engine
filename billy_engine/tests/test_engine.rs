#[cfg(test)]
mod teste_billy_engine {
	extern crate billy_engine;
	use billy_engine::engine::*;

	#[test]
	fn test_point() {
		const PX: i16 = 1;
		const PY: i16 = 2;
		let mut p = Point::new(1, 2);
		p.move_y(PX);
		assert_eq!(p.get_y(), 3);
		assert_eq!(p.get_x(), 1);
		p.set_y(PY);
		assert_eq!(p.get_y(), PY);
	}

	#[test]
	fn test_move_triangle() {
		let p1x =1;
		let p2x =10;

		let p1 = Point::new(p1x, p1x);
		let p2 = Point::new(p1x, p2x);
		let p3 = Point::new(p2x, p2x);

		let mut t = Triangle::new(
			p1,
			p2,
			p3
		);
		t.translate(1, 1);
		// Verif P1
		assert_eq!(p1x+1, t.get_point(0).get_x());
		assert_eq!(p1x+1, t.get_point(0).get_y());
		// Verif P2
		assert_eq!(p1x+1, t.get_point(1).get_x());
		assert_eq!(p2x+1, t.get_point(1).get_y());
		// Verif p3
		assert_eq!(p2x+1, t.get_point(2).get_x());
		assert_eq!(p2x+1, t.get_point(2).get_y());

		let mut t = Triangle::new(
			p1,
			p2,
			p3
		);

		t.translate_point(0, 2, 2);
		t.translate_point(1, 4, 4);
		t.translate_point(2, 1, 1);

		// Verif P1
		assert_eq!(p1x+2, t.get_point(0).get_x());
		assert_eq!(p1x+2, t.get_point(0).get_y());
		// Verif P2
		assert_eq!(p1x+4, t.get_point(1).get_x());
		assert_eq!(p2x+4, t.get_point(1).get_y());
		// Verif p3
		assert_eq!(p2x+1, t.get_point(2).get_x());
		assert_eq!(p2x+1, t.get_point(2).get_y());
	}
}