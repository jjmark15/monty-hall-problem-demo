use structopt::StructOpt;

use crate::application::ApplicationService;

pub(crate) fn run_cli(application_service: &ApplicationService) {
    let opts = CliOptions::from_args();
    let rate = application_service.calculate_switching_success_rate_for_iterations(opts.iterations);
    println!("rate: {}", rate);
}

/// Demo of the switching solution to the Monty Hall problem
#[derive(StructOpt, Debug)]
#[structopt(name = "monty-demo")]
struct CliOptions {
    /// Number of iterations to run
    #[structopt(short, long, default_value = "1")]
    iterations: u64,
}
