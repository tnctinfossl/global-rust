use std::net::UdpSocket;
use std::thread;
#[test]
fn recieve_tests() {
    let server_socket = UdpSocket::bind("224.5.23.2:10020")
                           .expect("Could not bind socket");
    loop {
        let mut buf = [0u8; 1024];
        // クロージャの中に移動されるからクローンする。
        let client_socket = server_socket.try_clone().expect("failed to clone socket");

        match server_socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("handling data from {}", src);
                    client_socket.send_to(&buf, src).expect("failed to send response");
                });
            },
            Err(e) => {
                eprintln!("could not recieve a datagram: {}", e);
            }
        }
    }
}



