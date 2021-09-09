use std::error::Error;
use structopt::StructOpt;

/// download stocks data and analysis for buy or sell.
#[derive(StructOpt)]
pub struct Opt {
    /// download data start date
    #[structopt(short = "s", long = "data-start-date", default_value = "2021-01-01")]
    data_start_date: String,

    /// download data end date
    #[structopt(short = "e", long = "data-end-date", default_value = "2021-09-01")]
    data_end_date: String,
}
#[derive(PartialEq, Debug)]
pub struct Config {
    pub data_start_date: String,
    pub data_end_date: String,
}

impl Config {
    pub fn new(args: Opt) -> Result<Config, String> {
        let data_start_date = args.data_start_date.clone();
        let data_end_date = args.data_end_date.clone();
        if data_start_date < "2020-01-01".to_string() {
            let mut result = String::from("data start date is error! ");
            result = result + &data_start_date;
            return Err(result);
        }

        Ok(Config {
            data_start_date,
            data_end_date,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{} {}", config.data_start_date, config.data_end_date);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config() {
        let args = Opt {
            data_start_date: String::from("2021-01-01"),
            data_end_date: String::from("2021-09-01"),
        };
        let config = Config::new(args).unwrap();

        assert_eq!(
            config,
            Config {
                data_start_date: String::from("2021-01-01"),
                data_end_date: String::from("2021-09-01"),
            }
        );

        assert_ne!(
            config,
            Config {
                data_start_date: String::from("2019-01-01"),
                data_end_date: String::from("2021-09-01"),
            }
        );
    }
}
