use rayon::prelude::*;

use crate::domain::contestant::Contestant;
use crate::domain::door::Door;
use crate::domain::game_show::GameShow;

pub(crate) struct SuccessRateCalculator {}

impl SuccessRateCalculator {
    pub(crate) fn new() -> Self {
        SuccessRateCalculator {}
    }

    pub(crate) fn calculate_switching_success_rate_for_iterations(&self, iterations: u64) -> f64 {
        let success_count = (0..iterations)
            .into_par_iter()
            .map(|_| self.demo_game_show_with_switching_method())
            .filter(|result| matches!(result, SwitchingDemoResult::Success))
            .count() as f64;
        success_count / iterations as f64
    }

    fn demo_game_show_with_switching_method(&self) -> SwitchingDemoResult {
        let mut game_show = Self::new_game_show();
        game_show.allow_contestant_to_choose_door();
        game_show.open_a_door_without_prize();
        game_show.allow_contestant_to_switch_doors();
        game_show.open_chosen_door();
        if game_show.contestant_has_won() {
            SwitchingDemoResult::Success
        } else {
            SwitchingDemoResult::Failure
        }
    }

    fn new_game_show() -> GameShow {
        let doors = [Door::new(Some(())), Door::new(None), Door::new(None)];
        let contestant = Contestant::new();
        GameShow::new(doors, contestant)
    }
}

enum SwitchingDemoResult {
    Success,
    Failure,
}
