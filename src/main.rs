mod libs;

use crate::libs::libs_billy_engine::billy_engine;
use crate::libs::libs_math::maths;
use std::{thread, time};


fn main() {
    maths::teste_macro();
    let seconde = time::Duration::from_millis(100);
    let _now = time::Instant::now();

    let mut engine = billy_engine::BillyEngine::new();
    let size: usize = engine.get_size();
    println!("{}", size);
    let mut pixel_buffer =  vec![' '; size];
    let v = billy_engine::Point::new(1, 0);
    let p0 = billy_engine::Point::new(1, 1);
    let p1 = billy_engine::Point::new(2, 2);
    let p2 = billy_engine::Point::new(2, 20);
    let p3 = billy_engine::Point::new(20, 20);
    let mut count = 1;

    let terminal_resolution: String = format!("V0.0.1A \t R{}X{}", engine.get_resolution().0, engine.get_resolution().1);
    let terminal_resolution: &str = terminal_resolution.as_str();
    let mut triangle = billy_engine::Triangle::new(p1,p2,p3);
    for _i in 0..208 {
        engine.clear(' ', &mut pixel_buffer);
        engine.put_texte(terminal_resolution, v, &mut pixel_buffer);
        engine.put_texte("Billy Engine Pour les Seigneurs de FE", p0, &mut pixel_buffer);
        engine.put_triangle(&triangle, &mut pixel_buffer);
        engine.draw(&mut pixel_buffer);
        thread::sleep(seconde);
        count+=1;
        println!("");
        triangle.translate_point(0, 1, 0);
        triangle.translate(1, 0);
    }
    println!("{}", count);
    println!("{:?}", engine.get_resolution());
}

