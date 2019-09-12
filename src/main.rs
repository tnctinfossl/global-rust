#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
mod settings;
//use arguments::arguments;

use std::env;
use std::fs::{File};
use std::io::{BufWriter};

fn main() {
    let args:Vec<String> = env::args().collect();
    //多分もっとうまくかける
    let subcommand :&str= if let Some(arg) = args.iter().nth(1){
        &arg
    } else {
        "help"
    };

    match subcommand{
        "help"=>{
            println!("usage:");
            println!("\tinit: create setting.json");
        },
        "init"=> {
            //多分もっとうまくかける
            let filename:&str = if let Some(name)=args.iter().nth(2){
                name
            }else{
                "setting.json"
            };
            
            let file = File::create(filename).expect("Error:Create Setting file");
            let  writer = BufWriter::new(file);
            let default =settings::Settings::default();
            serde_json::to_writer(writer, &default).expect("Error:Write Json");      
      
            
            


        },
        a=>{
            println!("{:?}",a)
        }
    }

}
