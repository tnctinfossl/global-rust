extern crate protoc_rust;

use protoc_rust::Customize;
use std::error::Error;

fn main ()->Result<(),Box<dyn Error>>{

    let proto_files=vec!["proto/messages_robocup_ssl_wrapper.proto",
        "proto/messages_robocup_ssl_refbox_log.proto",
        "proto/messages_robocup_ssl_geometry.proto",
        "proto/messages_robocup_ssl_detection.proto"
    ];
    let includes =vec!["proto"];
    protoc_rust::run(protoc_rust::Args{
        input:&proto_files[..],
        out_dir:"src/",
        includes:&includes[..],
        customize: Customize {
            ..Default::default()
        },
    })?;
    Ok(())
}

