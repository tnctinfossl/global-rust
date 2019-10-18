use super::messages_robocup_ssl_detection::*;
use super::messages_robocup_ssl_wrapper::SSL_WrapperPacket;
use glm::Vec2;
use log::warn;
use serde_derive::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::mpsc::Sender;
use std::thread;
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Settings {
    pub ip4: [u8; 4], //ip address  of cam or sim
    pub port: u16,    //
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            ip4: [224, 5, 23, 2],
            port: 10020,
        }
    }
}

pub struct Listener {
    sender: Sender<model::World>,
    socket: UdpSocket,
}

impl Listener {
    pub fn spawn(settings: &Settings,sender:Sender<model::World>) -> Result<(), String> {
        //socket open
        let addr = {
            let [a, b, c, d] = settings.ip4;
            Ipv4Addr::new(a, b, c, d)
        };
        let socket = UdpSocket::bind(&SocketAddr::from((addr, settings.port)))
            .map_err(|e| format!("Cannot bind vision server:{:?}", e))?;
        socket
            .join_multicast_v4(&addr, &Ipv4Addr::new(0, 0, 0, 0))
            .map_err(|e| format!("Cannot join vision server:{:?}", e))?;

        let listener = Listener {
            sender:sender,
            socket: socket,
        };

        thread::spawn(move || {
            let mut buffer = [0u8; 4096];
            loop {
                if let Err(e) = listener.receive(&mut buffer) {
                    warn!("{}", e);
                }
            }
        });

        Ok(())
    }

    fn receive(&self, buffer: &mut [u8]) -> Result<(), String> {
        let size = self
            .socket
            .recv(buffer)
            .map_err(|e| format!("Receive from vision server;{:?}", e))?;

        let packet: SSL_WrapperPacket = protobuf::parse_from_bytes(&buffer[..size])
            .map_err(|e| format! {"Parse from vision server;size={},{:?}", size, e})?;

        let detection = if packet.has_detection() {
            packet.get_detection()
        } else {
            return Ok(());
        };

        let balls: Vec<_> = detection
            .get_balls()
            .iter()
            .map(|ball| {
                Box::new(model::Ball::new(
                    Vec2::new(ball.get_x(), ball.get_y()),
                    ball.get_confidence(),
                ))
            })
            .collect();
        let robot_cast = |robot: &SSL_DetectionRobot| {
            let id = robot.get_robot_id();
            let position = Vec2::new(robot.get_x(), robot.get_y());
            let angle = if robot.has_orientation() {
                robot.get_orientation()
            } else {
                0.0
            };
            let confidence = robot.get_confidence();
            Box::new(model::Robot::new(id, position, angle, confidence))
        };

        let blues = model::Team {
            robots: detection.get_robots_blue().iter().map(robot_cast).collect(),
            ..model::Team::default()
        };

        let yellows = model::Team {
            robots: detection
                .get_robots_yellow()
                .iter()
                .map(robot_cast)
                .collect(),
            ..model::Team::default()
        };

        let world = model::World {
            balls: balls,
            blues: blues,
            yellows: yellows,
            ..model::World::default()
        };
        self.sender
            .send(world)
            .map_err(|e| format!("Vision:cannot send;{:?}", e))?;
        Ok(())
    }
}
