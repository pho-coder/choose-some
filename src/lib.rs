use structopt::StructOpt;

/// download stocks data and analysis for buy or sell.
#[derive(StructOpt)]
pub struct Config {
    /// download data start date
    #[structopt(default_value = "2021-01-01")]
    data_start_date: String,
    /// download data end date
    #[structopt(default_value = "2021-09-01")]
    data_end_date: String,
}

pub fn parse_config() -> Config {
    let args = Config::from_args();
    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_default() {
        let a = Config {"1","2"};
        assert_eq!(a, parse_config());
    }
}
