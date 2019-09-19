extern crate protobuf;
extern crate glm;
mod messages_robocup_ssl_detection;
mod messages_robocup_ssl_geometry;
mod messages_robocup_ssl_refbox_log;
mod messages_robocup_ssl_wrapper;
mod messages;

use messages::{Ball,Robot,World};
use listener::{Settings,listener};

#[macro_use]
extern crate log;
extern crate env_logger;