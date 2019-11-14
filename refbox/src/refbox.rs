use super::referee::{SSL_Referee, SSL_Referee_Command, SSL_Referee_Stage, SSL_Referee_TeamInfo};
use log::warn;
use model::{Command, Stage, Team, TeamColor};
use serde_derive::{Deserialize, Serialize};
use std::net::*;
use std::sync::mpsc::Sender;
use std::thread;
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Settings {
    pub ip4: [u8; 4],
    pub port: u16,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            ip4: [224, 5, 23, 1],
            port: 10003,
        }
    }
}

pub struct RefBox {
    socket: UdpSocket,
    sender: Sender<model::World>,
}

impl RefBox {
    pub fn spawn(settings: &Settings, sender: Sender<model::World>) -> Result<(), String> {
        //multicastを受け付ける
        let addr = Ipv4Addr::from(settings.ip4);
        let addr_port = (addr, settings.port);
        let socket =
            UdpSocket::bind(addr_port).map_err(|e| format!("refbox cannnot bind;{:?}", e))?;
        socket
            .join_multicast_v4(&addr, &Ipv4Addr::from([0, 0, 0, 0]))
            .map_err(|e| format!("refbox cannnot join multicast;{:?}", e))?;
        let refbox = RefBox {
            socket: socket,
            sender: sender,
        };
        thread::spawn(move || {
            let mut buffer = [0; 1024];
            loop {
                if let Err(e) = refbox.recive(&mut buffer) {
                    warn!("{}", e);
                }
            }
        });
        Ok(())
    }

    fn recive(&self, buffer: &mut [u8]) -> Result<(), String> {
        let size = self
            .socket
            .recv(buffer)
            .map_err(|e| format!("refbox failure receiving;{:?}", e))?;
        let referee: SSL_Referee = protobuf::parse_from_bytes(&buffer[..size])
            .map_err(|e| format!("refbox failure parsing;{:?}", e))?;

        let team_cast = |info: &SSL_Referee_TeamInfo| model::Team {
            name: Some(info.get_name().to_owned()),
            red_card: Some(info.get_red_cards()),
            yellow_card: Some(info.get_yellow_cards()),
            score: Some(info.get_score()),
            goalie: Some(info.get_goalie()),
            ..model::Team::default()
        };

        let world = model::World {
            blues: team_cast(referee.get_blue()),
            yellows: team_cast(referee.get_yellow()),
            stage: Some(RefBox::to_stage(referee.get_stage())),
            command: Some(RefBox::to_command(referee.get_command())),
            ..model::World::default()
        };
        self.sender
            .send(world)
            .map_err(|e| format!("refbox cannot send;{:?}", e))?;

        Ok(())
    }

    fn to_stage(stage: SSL_Referee_Stage) -> Stage {
        use SSL_Referee_Stage::*;
        use Stage::*;
        match stage {
            NORMAL_FIRST_HALF_PRE => NormalFirstHalfPre,
            NORMAL_FIRST_HALF => NormalFirstHalf,
            NORMAL_HALF_TIME => NormalHalfTime,
            NORMAL_SECOND_HALF_PRE => NormalSecondHalfPre,
            NORMAL_SECOND_HALF => NormalSecondHalf,
            EXTRA_TIME_BREAK => ExtraTimeBreak,
            EXTRA_FIRST_HALF_PRE => ExtraFirstHalfPre,
            EXTRA_FIRST_HALF => ExtraFirstHalf,
            EXTRA_HALF_TIME => ExtraHalfTime,
            EXTRA_SECOND_HALF_PRE => ExtraSecondHalfPre,
            EXTRA_SECOND_HALF => ExtraSecondHalf,
            PENALTY_SHOOTOUT_BREAK => PenaltyShootoutBreak,
            PENALTY_SHOOTOUT => PenaltyShootout,
            POST_GAME => PostGame,
        }
    }
    fn to_command(command: SSL_Referee_Command) -> Command {
        use Command::*;
        use SSL_Referee_Command::*;
        use TeamColor::*;
        match command {
            HALT => Halt,
            STOP => Stop,
            NORMAL_START => NormalStart,
            FORCE_START => ForceStart,
            PREPARE_KICKOFF_YELLOW => PrepareKickOff(Yellow),
            PREPARE_KICKOFF_BLUE => PrepareKickOff(Blue),
            PREPARE_PENALTY_YELLOW => PreparePenalty(Yellow),
            PREPARE_PENALTY_BLUE => PreparePenalty(Blue),
            DIRECT_FREE_YELLOW => DirectFree(Yellow),
            DIRECT_FREE_BLUE => DirectFree(Blue),
            INDIRECT_FREE_YELLOW => IndirectFree(Yellow),
            INDIRECT_FREE_BLUE => IndirectFree(Blue),
            TIMEOUT_YELLOW => Timeout(Yellow),
            TIMEOUT_BLUE => Timeout(Blue),
            GOAL_YELLOW => Goal(Yellow),
            GOAL_BLUE => Goal(Blue),
            BALL_PLACEMENT_YELLOW => BallPlacement(Yellow),
            BALL_PLACEMENT_BLUE => BallPlacement(Blue),
        }
    }

    
    fn update_team(team: &mut Team, info: &SSL_Referee_TeamInfo) {}
}
