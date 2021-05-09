use structopt::StructOpt;

use crate::application::ApplicationService;
use crate::ports::cli::structopt::solution_method::SolutionMethod;

mod solution_method;

pub(crate) fn run_cli(application_service: &ApplicationService) {
    let opts = CliOptions::from_args();
    let rate = calculate_success_rate_for_method_and_iterations(
        opts.method,
        opts.iterations,
        application_service,
    );

    println!("success rate: {}", rate);
}

fn calculate_success_rate_for_method_and_iterations(
    solution_method: SolutionMethod,
    iteration_count: u64,
    application_service: &ApplicationService,
) -> f64 {
    match solution_method {
        SolutionMethod::Switch => {
            application_service.calculate_switching_success_rate_for_iterations(iteration_count)
        }
        SolutionMethod::Stick => {
            application_service.calculate_sticking_success_rate_for_iterations(iteration_count)
        }
        SolutionMethod::Random => {
            application_service.calculate_random_success_rate_for_iterations(iteration_count)
        }
    }
}

/// Demo of the switching solution to the Monty Hall problem
#[derive(StructOpt, Debug)]
#[structopt(name = "monty-demo")]
struct CliOptions {
    /// Number of iterations to run
    #[structopt(short, long, default_value = "1")]
    iterations: u64,

    /// Method for contestant to follow
    #[structopt(short, long, default_value = "switch")]
    method: SolutionMethod,
}
