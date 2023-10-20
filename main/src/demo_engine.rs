
extern crate billy_engine;
use std::{thread, time};

use billy_engine::maths;
use billy_engine::engine::{Point, BillyEngine, Triangle};

pub fn demo_triangle() {
	maths::teste_macro();

	let seconde = time::Duration::from_millis(100);
    let _now = time::Instant::now();

    let mut engine = BillyEngine::new();
    let v = Point::new(1, 0);
    let p0 = Point::new(1, 1);
    let p1 = Point::new(2, 2);
    let p2 = Point::new(2, 20);
    let p3 = Point::new(20, 20);
    let mut count = 1;

    let terminal_resolution: String = format!("V0.0.1A   R{}X{}", engine.get_resolution().0, engine.get_resolution().1);
    let terminal_resolution: &str = terminal_resolution.as_str();
    let mut triangle = Triangle::new(p1,p2,p3);
    for _i in 0..209 {
        engine.clear('-');
        engine.put_texte(terminal_resolution, v);
        engine.put_texte("Billy Engine Pour les Seigneurs de FE", p0);
        engine.put_triangle(&triangle);
        engine.draw();
        thread::sleep(seconde);
        count+=1;
        triangle.translate_point(0, 1, 0);
        triangle.translate(1, 0);
    }
    println!("{}", count);
    println!("{:?}", engine.get_resolution());
}