use structopt::clap::arg_enum;

arg_enum! {
    #[derive(Debug)]
    pub enum SolutionMethod {
        Switch,
        Stick,
        Random,
    }
}
