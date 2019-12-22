#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

extern crate log;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
pub mod packet;
pub mod radio;
pub use packet::*;
pub use radio::*;
