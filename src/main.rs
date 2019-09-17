mod receiver;
mod settings;
mod viewer;
mod subprograms;
use receiver::receiver::Receiver;
use settings::Settings;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::net::Ipv4Addr;
use subprograms::{SubProgram, SubPrograms, SubProgramsTrait};
use viewer::Viewer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let subprograms: SubPrograms = vec![
        SubProgram::new(
            "init".to_string(),
            "[filename]".to_string(),
            "create settings file".to_string(),
            Box::new(init),
        ),
        SubProgram::new(
            "receive".to_string(),
            "[filename]".to_string(),
            "receive test from servers".to_string(),
            Box::new(receive),
        ),
        SubProgram::new(
            "run".to_string(),
            "[filename]".to_string(),
            "run".to_string(),
            Box::new(run),
        ),
    ];

    subprograms.run(&args[1..]);
}

fn init(args: &[String]) {
    //TODO きれいに書く
    let filename: &str = if let Some(name) = args.iter().nth(1) {
        name
    } else {
        "setting.json"
    };
    let file = File::create(filename).expect(&format!("Error:Cannot Create {}", filename));
    let writer = BufWriter::new(file);
    let default = Settings::default();
    serde_json::to_writer(writer, &default).expect(&format!("Error:Cannot Write {}", filename));
}

fn receive(args: &[String]) {
    //simple receiver
    let filename: &str = if let Some(name) = args.iter().nth(1) {
        name
    } else {
        "setting.json"
    };

    let file = File::open(filename).expect(&format!("Error:Cannot Read {}", filename));
    let reader = BufReader::new(file);
    let settings: Settings =
        serde_json::from_reader(reader).expect(&format!("Error:Cannot Parse {}", filename));

    let ip = {
        let [a, b, c, d] = settings.net.vision_ip4;
        Ipv4Addr::new(a, b, c, d)
    };
    let mut r = Receiver::open(ip, settings.net.vision_port).unwrap();
    r.recv().unwrap();
}

fn run(args: &[String]) {
    let filename: &str = if let Some(name) = args.iter().nth(1) {
        name
    } else {
        "setting.json"
    };

    let file = File::open(filename).expect(&format!("Error:Cannot Read {}", filename));
    let reader = BufReader::new(file);
    let settings: Settings =
        serde_json::from_reader(reader).expect(&format!("Error:Cannot Parse {}", filename));

    let ip = {
        let [a, b, c, d] = settings.net.vision_ip4;
        Ipv4Addr::new(a, b, c, d)
    };
    let mut r = Receiver::open(ip, settings.net.vision_port).unwrap();


    Viewer::new(&settings.viewer);




}
