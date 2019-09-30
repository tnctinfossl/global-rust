
use listener::{Listener,Settings};
fn main(){
    //test code
    let settings=Settings::default();
    let listener=Listener::new(&settings);
    let receiver=&listener.world_receiver;
    loop{
        if let Ok(world)=receiver.try_recv(){
            println!("{:?}",world);
        }
    }
}