use std::sync::{Mutex, Arc};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
	static ref LE: Mutex<ListEvent> = Mutex::new(
		ListEvent::new()
	);
}

type Event = String;
/// Gestionaire d'évènement de l'application
pub struct ListEvent {
    subscriptions: Arc<Mutex<HashMap<Event, Vec<Box<dyn Fn() + Send>>>>>,
}

impl ListEvent {
    pub fn new() -> Self {
        ListEvent {
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

	/// Permet de ce souscrire à un évènement et le crée si non existant
	/// 'event' Nom de l'évènement
	/// 'callback' méthode à lier à l'évènement
    pub fn subscribe<F>(&self, event: &str, callback: F)
    where
        F: Fn() + Send + 'static,
    {
        let mut subscriptions = self.subscriptions.lock().unwrap();
        if let Some(add_call) = subscriptions.get_mut(event){
			add_call.push(Box::new(callback));
        } else {
            let v: Vec<Box<dyn Fn() + Send>> = vec![Box::new(callback)];
            subscriptions
            	.entry(event.to_string())
            	.or_insert(v);
        }
    }

	///Appelle un évènement
	/// 'event' &str nom de l'evencement
    pub fn publish(&self, event: &str) {
        let subscriptions = self.subscriptions.lock().unwrap();
        if let Some(callbacks) = subscriptions.get(event) {
            println!("o");
			for callback in callbacks.iter() {
                callback();
            }
        }
    }
}

/// Permet de ce souscrit au gestionaire d'evènements Global
pub fn subscribe<F>(name: &str, c: F)
	where
		F: Fn() + Send + 'static,
	{
	let le = LE.lock().unwrap();
	le.subscribe(name, c);
}

/// Appelle un évènement
pub fn publish(name: &str) {
	let le = LE.lock().unwrap();
	le.publish(name);
}
