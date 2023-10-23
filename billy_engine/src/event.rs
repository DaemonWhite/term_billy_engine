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

///
pub fn subscribe<F>(name: &str, c: F)
	where
		F: Fn() + Send + 'static,
	{
	let le = LE.lock().unwrap();
	le.subscribe(name, c);
}

pub fn publish(name: &str) {
	let le = LE.lock().unwrap();
	le.publish(name);
}
