

use crate::state::counter::{Counter, COUNTER};

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn new_counter()->i32 {
    let mut r = COUNTER.write().unwrap();
    *r = Some(Counter { data: 0 });

    return 0;
}

pub fn add() -> i32 {
    let mut r = COUNTER.write().unwrap();
    // (*r).as_mut().unwrap().minus();
    match (*r).as_mut() {
        Some(_r) => {
            _r.add();
            return _r.data;
        },
        None => {
            0
        },
    }
}

pub fn minus() -> i32 {
    let mut r = COUNTER.write().unwrap();
    // (*r).as_mut().unwrap().minus();
    match (*r).as_mut() {
        Some(_r) => {
            _r.minus();
            return _r.data;
        },
        None => {
            0
        },
    }
}

// pub async fn refresh_counter(dart_callback: impl Fn(i32) -> DartFnFuture<i32>) {
//     let r = COUNTER.read().unwrap();
//     match r.clone() {
//         Some(_r) => {
//             dart_callback(_r.data).await;
//         }
//         None => {}
//     }
// }
