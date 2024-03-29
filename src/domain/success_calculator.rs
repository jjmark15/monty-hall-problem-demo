use rayon::iter::repeatn;
use rayon::prelude::*;
use std::collections::HashMap;

use crate::domain::contestant::Contestant;
use crate::domain::door::Door;
use crate::domain::game_show::GameShow;

pub(crate) struct SuccessRateCalculator {}

impl SuccessRateCalculator {
    pub(crate) fn new() -> Self {
        SuccessRateCalculator {}
    }

    pub(crate) fn calculate_success_rates(&self, iterations: usize) -> HashMap<String, f64> {
        let stick_rate = self.calculate_sticking_success_rate_for_iterations(iterations);
        HashMap::from_iter([
            (
                "random".to_string(),
                self.calculate_random_success_rate_for_iterations(iterations),
            ),
            ("stick".to_string(), stick_rate),
            ("switch".to_string(), 1_f64 - stick_rate),
        ])
    }

    pub(crate) fn calculate_sticking_success_rate_for_iterations(
        &self,
        iteration_count: usize,
    ) -> f64 {
        self.calculate_method_success_rate_for_iterations(
            iteration_count,
            Self::demo_game_show_with_sticking_method,
        )
    }

    pub(crate) fn calculate_random_success_rate_for_iterations(
        &self,
        iteration_count: usize,
    ) -> f64 {
        self.calculate_method_success_rate_for_iterations(
            iteration_count,
            Self::demo_game_show_with_random_method,
        )
    }

    fn calculate_method_success_rate_for_iterations<F>(
        &self,
        iteration_count: usize,
        solution_function: F,
    ) -> f64
    where
        F: Fn() -> DemoResult + Sync + Send,
    {
        let success_count = repeatn((), iteration_count)
            .map(|_| solution_function())
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
