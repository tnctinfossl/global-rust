use super::referee::{SSL_Referee, SSL_Referee_Command, SSL_Referee_Stage};
use model::{Stage, World};
pub struct Updater;

impl Updater {
    pub fn new() -> Updater {
        Updater {}
    }

    pub fn update(&self, world: &mut World, referee: &SSL_Referee) {
        world.stage = Some(Updater::to_stage(referee.get_stage()));
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
}
