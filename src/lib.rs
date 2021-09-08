use std::error::Error;
use structopt::StructOpt;

/// download stocks data and analysis for buy or sell.
#[derive(StructOpt)]
struct Opt {
    /// download data start date
    #[structopt(default_value = "2021-01-01")]
    data_start_date: String,
    /// download data end date
    #[structopt(default_value = "2021-09-01")]
    data_end_date: String,
}

pub struct Config {
    pub data_start_date: String,
    pub data_end_date: String,
}

impl Config {
    pub fn new() -> Result<Config, &str> {
        let args = Opt::from_args();
        println!("{}", args.data_start_date);
        if 1 > 2 {
            return Err("I don't known!");
        }

        let data_start_date = "12";
        let data_end_date = "2323";

        Ok(Config {
            data_start_date,
            data_end_date,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", config.data_start_date);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config_default() {
        let config = Config {
            data_start_date: "2".to_string(),
            data_end_date: "2".to_string(),
        };
        assert_eq!(1, 1);
    }
}
