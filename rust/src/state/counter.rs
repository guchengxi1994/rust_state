use std::sync::RwLock;

use once_cell::sync::Lazy;

use super::State;

pub type Counter = State<i32>;

impl Counter {
    pub fn add(&mut self) {
        println!("data : {:?}",self.data);
        self.data += 1;
    }

    pub fn minus(&mut self) {
        println!("data : {:?}",self.data);
        self.data -= 1;
    }
}

pub static COUNTER: Lazy<RwLock<Option<Counter>>> = Lazy::new(|| RwLock::new(None));
