
extern crate billy_engine;
use std::{thread, time::{Duration, Instant}};
use std::sync::{Arc, Mutex};

use billy_engine::maths;
use billy_engine::engine::{Point, BillyEngine, Triangle};
use billy_engine::ui::{Boxe, BoxeElement};
// TODO Créer des paramètres avec

pub static mut MENU_CHOICE: usize = 0;

pub fn menu_choice(choice: usize) {
	unsafe {
		MENU_CHOICE = choice;
	}
}


pub fn menu(engine: Arc<Mutex<BillyEngine>>, choice_demo: Arc<Mutex<BoxeElement>>, bijour: Boxe ) {
	thread::sleep(Duration::from_millis(100));
	let choice_demo = choice_demo.lock().unwrap();
	let mut engine = engine.lock().unwrap();
	engine.clear(' ');
	engine.put_object(choice_demo.clone());
	engine.put_object(bijour.clone());
	engine.draw();
}

pub fn demo_triangle(engine: Arc<Mutex<BillyEngine>>) {
	let seconde = Duration::from_millis(100);
    let _now = Instant::now();
    let v = Point::new(1, 0);
    let p0 = Point::new(1, 1);
    let p1 = Point::new(2, 2);
    let p2 = Point::new(2, 20);
    let p3 = Point::new(20, 20);
    let mut count = 1;
    let clone_engine = Arc::clone(&engine);
    let tmp_engine = engine.lock().unwrap();
    let terminal_resolution: String = format!("V0.0.1A   R{}X{}", tmp_engine.get_resolution().0, tmp_engine.get_resolution().1);
    let terminal_resolution: &str = terminal_resolution.as_str();
    let mut triangle = Triangle::new(p1,p2,p3);
    drop(tmp_engine);
    for _i in 0..200 {
		let engine = Arc::clone(&clone_engine);
		let mut engine = engine.lock().unwrap();
        engine.clear('-');
        engine.put_texte(terminal_resolution, v);
        engine.put_texte("Billy Engine Pour les Seigneurs de FE", p0);
        engine.put_triangle(&triangle);
        engine.draw();
        drop(engine);
        thread::sleep(seconde);
        count+=1;
        triangle.translate_point(0, 1, 0);
        triangle.translate(1, 0);
    }
    println!("nb cicle : {}\r", count);
    thread::sleep(Duration::from_secs(5));
    menu_choice(0);
}

pub fn info(engine: Arc<Mutex<BillyEngine>>) {
    let mut engine = engine.lock().unwrap();
	let terminal_resolution: String = format!("V0.0.1A   R{}X{}", engine.get_resolution().0, engine.get_resolution().1);
    let terminal_resolution: &str = terminal_resolution.as_str();
    let v = Point::new(1, 0);
    engine.clear('-');
    engine.put_texte(terminal_resolution, v);
    engine.draw();
    maths::teste_macro();
	thread::sleep(Duration::from_secs(10));
	menu_choice(0);
}