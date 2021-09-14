use std::env;
use structopt::StructOpt;

mod crawl_data;
use crawl_data::crawl;

/// download stocks data and analysis for buy or sell.
#[derive(StructOpt)]
pub struct Opt {
    /// download data start date
    #[structopt(short = "s", long = "data-start-date", default_value = "20210101")]
    data_start_date: String,

    /// download data end date
    #[structopt(short = "e", long = "data-end-date", default_value = "20210901")]
    data_end_date: String,
}
#[derive(PartialEq, Debug)]
pub struct Config {
    pub data_start_date: String,
    pub data_end_date: String,
    pub tushare_token: String,
    pub data_dir: String,
}

impl Config {
    pub fn new(args: Opt) -> Result<Config, String> {
        let data_start_date = args.data_start_date.clone();
        let data_end_date = args.data_end_date.clone();
        if data_start_date < "20200101".to_string() {
            let mut result = String::from("data start date is error! ");
            result = result + &data_start_date;
            return Err(result);
        }

        let tushare_token = env::var("TUSHARE_TOKEN").unwrap();
        if tushare_token.eq("") {
            return Err(String::from("NO TUSHARE_TOKEN!"));
        }

        let data_dir = env::var("DATA_DIR").unwrap();
        if data_dir.eq("") {
            return Err(String::from("NO DATA_DIR!"));
        }

        Ok(Config {
            data_start_date,
            data_end_date,
            tushare_token,
            data_dir,
        })
    }
}

pub fn run(config: &Config) -> Result<(), String> {
    println!("{} {}", config.data_start_date, config.data_end_date);
    if let Err(e) = crawl::run(config) {
        eprintln!("Application error: {}", e);
        return Err("crawl error!".to_owned());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config() {
        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: String::from("20210901"),
        };
        let config = Config::new(args).unwrap();
        let tushare_token = env::var("TUSHARE_TOKEN").unwrap();
        let data_dir = env::var("DATA_DIR").unwrap();

        assert_eq!(
            config,
            Config {
                data_start_date: String::from("20210101"),
                data_end_date: String::from("20210901"),
                tushare_token: tushare_token,
                data_dir: data_dir,
            }
        );

        assert_ne!(
            config,
            Config {
                data_start_date: String::from("20190101"),
                data_end_date: String::from("20210901"),
                tushare_token: String::from(""),
                data_dir: String::from(""),
            }
        );
    }
}
