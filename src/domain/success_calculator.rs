use rayon::prelude::*;

use crate::domain::contestant::Contestant;
use crate::domain::door::Door;
use crate::domain::game_show::GameShow;

pub(crate) struct SuccessRateCalculator {}

impl SuccessRateCalculator {
    pub(crate) fn new() -> Self {
        SuccessRateCalculator {}
    }

    pub(crate) fn calculate_switching_success_rate_for_iterations(
        &self,
        iteration_count: u64,
    ) -> f64 {
        let switching_success_count = (0..iteration_count)
            .into_par_iter()
            .map(|_| self.demo_game_show_with_switching_method())
            .filter(SwitchingDemoResult::is_success)
            .count() as f64;
        switching_success_count / iteration_count as f64
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
        GameShow::new(
            [Door::new(Some(())), Door::new(None), Door::new(None)],
            Contestant::new(),
        )
    }
}

enum SwitchingDemoResult {
    Success,
    Failure,
}

impl SwitchingDemoResult {
    fn is_success(&self) -> bool {
        matches!(self, SwitchingDemoResult::Success)
    }
}
