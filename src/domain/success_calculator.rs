use rayon::prelude::*;

use crate::domain::contestant::Contestant;
use crate::domain::door::Door;
use crate::domain::game_show::GameShow;
use crate::domain::solution_method::SolutionMethod;

pub(crate) struct SuccessRateCalculator {}

impl SuccessRateCalculator {
    pub(crate) fn new() -> Self {
        SuccessRateCalculator {}
    }

    pub(crate) fn calculate_switching_success_rate_for_iterations(
        &self,
        iteration_count: u64,
    ) -> f64 {
        self.calculate_method_success_rate_for_iterations(SolutionMethod::Switch, iteration_count)
    }

    pub(crate) fn calculate_sticking_success_rate_for_iterations(
        &self,
        iteration_count: u64,
    ) -> f64 {
        self.calculate_method_success_rate_for_iterations(SolutionMethod::Stick, iteration_count)
    }

    pub(crate) fn calculate_random_success_rate_for_iterations(&self, iteration_count: u64) -> f64 {
        self.calculate_method_success_rate_for_iterations(SolutionMethod::Random, iteration_count)
    }

    fn calculate_method_success_rate_for_iterations(
        &self,
        solution_method: SolutionMethod,
        iteration_count: u64,
    ) -> f64 {
        let success_count = (0..iteration_count)
            .into_par_iter()
            .map(|_| match solution_method {
                SolutionMethod::Switch => Self::demo_game_show_with_switching_method(),
                SolutionMethod::Stick => Self::demo_game_show_with_sticking_method(),
                SolutionMethod::Random => Self::demo_game_show_with_random_method(),
            })
            .filter(DemoResult::is_success)
            .count() as f64;
        success_count / iteration_count as f64
    }

    fn demo_game_show_with_switching_method() -> DemoResult {
        let mut game_show = Self::new_game_show();
        game_show.allow_contestant_to_choose_door();
        game_show.open_a_door_without_prize();
        game_show.allow_contestant_to_switch_doors();
        game_show.open_chosen_door();
        if game_show.contestant_has_won() {
            DemoResult::Success
        } else {
            DemoResult::Failure
        }
    }

    fn demo_game_show_with_sticking_method() -> DemoResult {
        match Self::demo_game_show_with_switching_method() {
            DemoResult::Success => DemoResult::Failure,
            DemoResult::Failure => DemoResult::Success,
        }
    }

    fn demo_game_show_with_random_method() -> DemoResult {
        let mut game_show = Self::new_game_show();
        game_show.allow_contestant_to_choose_door();
        game_show.open_a_door_without_prize();
        game_show.allow_contestant_to_choose_door();
        game_show.open_chosen_door();
        if game_show.contestant_has_won() {
            DemoResult::Success
        } else {
            DemoResult::Failure
        }
    }

    fn new_game_show() -> GameShow {
        GameShow::new(
            [
                Door::with_prize(),
                Door::without_prize(),
                Door::without_prize(),
            ],
            Contestant::new(),
        )
    }
}

enum DemoResult {
    Success,
    Failure,
}

impl DemoResult {
    fn is_success(&self) -> bool {
        matches!(self, DemoResult::Success)
    }
}
