extern crate protoc_rust;

use protoc_rust::Customize;
use std::error::Error;

fn main ()->Result<(),Box<dyn Error>>{
    let proto_files=vec!["src/proto/grSim_Packet.proto","src/proto/grSim_Replacement.proto","src/proto/grSim_Commands.proto"];
    let includes =vec!["src/proto"];
    protoc_rust::run(protoc_rust::Args{
        input:&proto_files[..],
        out_dir:"src/receiver",
        includes:&includes[..],
        customize: Customize {
            ..Default::default()
        },
    })?;
    Ok(())
}

