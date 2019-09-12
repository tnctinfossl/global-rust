mod settings;
mod subprograms;

use std::env;
use std::fs::File;
use std::io::BufWriter;
use subprograms::{SubProgram,SubPrograms,SubProgramsTrait};

fn main() {
    let subprograms: SubPrograms = vec![SubProgram::new(
        "init".to_string(),
        "[filename]".to_string(),
        "create settings file".to_string(),
        Box::new(init))
    ];

    let args: Vec<String> = env::args().collect();

    subprograms.run(&args[1..]);
}

fn init(args: &[String]) {
    //TODO きれいに書く
    let filename: &str = if let Some(name) = args.iter().nth(2) {
        name
    } else {
        "setting.json"
    };

    let file = File::create(filename).expect("Error:Create Setting file");
    let writer = BufWriter::new(file);
    let default = settings::Settings::default();
    serde_json::to_writer(writer, &default).expect("Error:Write Json");
}

fn receive(args: &[String]) {}
