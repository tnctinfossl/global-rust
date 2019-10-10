use std::sync::{Arc,RwLock};
use vision::{Listener,Settings};
fn main(){
    //test code
    let settings=Settings::default();

    let world = Arc::new(RwLock::new(model::World::default()));
   
    Listener::spawn(&settings,world);
    loop{}
}