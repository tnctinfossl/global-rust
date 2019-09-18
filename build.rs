extern crate protoc_rust;

use protoc_rust::Customize;
use std::error::Error;

fn main ()->Result<(),Box<dyn Error>>{

    
    let proto_files=vec!["src/proto/messages_robocup_ssl_wrapper.proto",
        "src/proto/messages_robocup_ssl_refbox_log.proto",
        "src/proto/messages_robocup_ssl_geometry.proto",
        "src/proto/messages_robocup_ssl_detection.proto"
    ];
    let includes =vec!["src/proto"];
    protoc_rust::run(protoc_rust::Args{
        input:&proto_files[..],
        out_dir:"src/listener",
        includes:&includes[..],
        customize: Customize {
            ..Default::default()
        },
    })?;
    Ok(())
}

