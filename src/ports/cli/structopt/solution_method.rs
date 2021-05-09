use std::str::FromStr;

#[derive(Debug)]
pub(crate) enum SolutionMethod {
    Switch,
}

impl FromStr for SolutionMethod {
    type Err = SolutionMethodParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let method = match s.to_lowercase().as_str() {
            "switch" => SolutionMethod::Switch,
            _ => return Err(SolutionMethodParseError(s.to_string())),
        };
        Ok(method)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Could not parse solution method from '{0}'")]
pub(crate) struct SolutionMethodParseError(String);
