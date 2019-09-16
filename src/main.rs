mod settings;
mod subprograms;
use settings::Settings;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::net::{UdpSocket,Ipv4Addr,SocketAddr};
use subprograms::{SubProgram, SubPrograms, SubProgramsTrait};

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
/*
    let ipv4= {
        let ip=&settings.vision_ip4;
        Ipv4Addr::new(ip[0],ip[1],ip[2],ip[3])
    };
*/
    let vision_mount =SocketAddr::from((settings.vision_ip4,settings.vision_port));
    let socket = UdpSocket::bind(&vision_mount).expect("Error:Cannot Bind");
    socket.join_multicast_v4(&Ipv4Addr::new(224,5,23,2), &Ipv4Addr::new(0,0,0,0)).expect("Error:Cannot: multicast");

    let mut buf= [0;1024];
    let (amt,_)=socket.recv_from(&mut buf).expect("Error:Cannot recv");
    println!("size:{}",amt);
    for idx in 0..amt{
        print!("{},",buf[idx]);
    }  



}
