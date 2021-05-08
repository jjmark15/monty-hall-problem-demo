use crate::domain::SuccessRateCalculator;

pub(crate) struct ApplicationService {}

impl ApplicationService {
    pub(crate) fn new() -> Self {
        ApplicationService {}
    }

    pub(crate) fn calculate_switching_success_rate_for_iterations(
        &self,
        iteration_count: u64,
    ) -> f64 {
        let success_rate_calculator = SuccessRateCalculator::new();
        success_rate_calculator.calculate_switching_success_rate_for_iterations(iteration_count)
    }
}
