use super::referee::{SSL_Referee, SSL_Referee_Command, SSL_Referee_Stage,SSL_Referee_TeamInfo};
use model::{Command, Stage, TeamColor, World,Team};
use std::time;
pub struct Updater;

impl Updater {
    pub fn new() -> Updater {
        Updater {}
    }

    pub fn update(&self, world: &mut World, referee: &SSL_Referee) {
        world.stage = Some(Updater::to_stage(referee.get_stage()));
        world.command = Some(Updater::to_command(referee.get_command()));
        Updater::update_team(&mut world.blues, referee.get_blue());
        Updater::update_team(&mut world.yellows, referee.get_yellow());
        world.timestamp=time::Instant::now();
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

    fn update_team(team:&mut Team,info:&SSL_Referee_TeamInfo){
        team.name=info.get_name().to_owned();
        team.red_card=info.get_red_cards();
        team.yellow_card=info.get_yellow_cards();
        team.score=info.get_score();
        team.goalie=info.get_goalie();
    }
}
