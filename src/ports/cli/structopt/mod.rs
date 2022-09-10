use structopt::StructOpt;

use crate::domain::SuccessRateCalculator;

mod solution_method;

pub(crate) fn run_cli() {
    let opts = CliOptions::from_args();

    let calculator = SuccessRateCalculator::new();
    let success_counts = calculator.calculate_success_rates(opts.iterations);

    let mut output_strings = success_counts
        .into_iter()
        .map(|(method_name, count)| format!("{}: {}", method_name, count))
        .collect::<Vec<String>>();
    output_strings.sort();

    println!("{}", output_strings.join("\n"));
}

/// Demo of the switching solution to the Monty Hall problem
#[derive(StructOpt, Debug)]
#[structopt(name = "monty-demo")]
struct CliOptions {
    /// Number of iterations to run
    #[structopt(short, long, default_value = "1")]
    iterations: usize,
}
