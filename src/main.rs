#[macro_use]
mod message;
mod arguments;
//use message::messages_robocup_ssl_detection as direction;
//use message::messages_robocup_ssl_geometry as geometry;
//use message::messages_robocup_ssl_refbox_log as refbox;
//use message::messages_robocup_ssl_wrapper as wrapper;
use arguments::*;

fn main() {
    let result =parse();
    println!("{:?}",result);
}
