use std::env;
use std::str::FromStr;
use structopt::StructOpt;

mod analysis;
mod crawl;
mod models;

/// download stocks data and analysis for buy or sell.
#[derive(StructOpt)]
pub struct Opt {
    /// download data start date
    #[structopt(short = "s", long = "data-start-date", default_value = "20210101")]
    data_start_date: String,

    /// download data end date
    #[structopt(short = "e", long = "data-end-date", default_value = "20210901")]
    data_end_date: String,

    /// download data type
    #[structopt(short = "t", long = "download-type", default_value = "all")]
    download_type: DownloadType,
}
#[derive(Debug, PartialEq)]
pub struct Config {
    pub data_start_date: String,
    pub data_end_date: String,
    pub tushare_token: String,
    pub data_dir: String,
    pub download_type: DownloadType,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DownloadType {
    All,
    Daily,
    DailyBasic,
}
type ParseError = &'static str;
impl FromStr for DownloadType {
    type Err = ParseError;
    fn from_str(download_type: &str) -> Result<Self, Self::Err> {
        match download_type {
            "daily" => Ok(DownloadType::Daily),
            "daily_basic" => Ok(DownloadType::DailyBasic),
            "all" => Ok(DownloadType::All),
            _ => Err("Could not parse download-type"),
        }
    }
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

        let download_type = args.download_type;

        Ok(Config {
            data_start_date,
            data_end_date,
            tushare_token,
            data_dir,
            download_type,
        })
    }
}

pub fn run(config: &mut Config) -> Result<(), String> {
    println!("{} {}", config.data_start_date, config.data_end_date);
    let (earliest_trade_date, latest_trade_date) = crawl::run(config).unwrap();
    config.data_start_date = earliest_trade_date;
    config.data_end_date = latest_trade_date;
    println!("{} {}", config.data_start_date, config.data_end_date);
    analysis::run(config).unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn parse_config() {
        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: String::from("20210901"),
            download_type: DownloadType::All,
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
                download_type: DownloadType::All,
            }
        );

        assert_ne!(
            config,
            Config {
                data_start_date: String::from("20190101"),
                data_end_date: String::from("20210901"),
                tushare_token: String::from(""),
                data_dir: String::from(""),
                download_type: DownloadType::All,
            }
        );
    }
}
