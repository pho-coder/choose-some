use crate::Config;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

fn _test_type<T>(_: T) {
    println!("{:?}", { type_name::<T>() });
}

#[derive(Debug)]
struct MyError(String);
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for MyError {}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    info!("{} {}", config.data_start_date, config.data_end_date);
    let data_dir = Path::new(&config.data_dir);
    let trade_date = get_latest_trade_cal(config).unwrap();

    init_dir(data_dir, trade_date)?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct TushareRESTfulAPI {
    api_name: String,
    token: String,
    params: HashMap<String, String>,
    fields: String,
}

fn get_latest_trade_cal(config: &Config) -> Result<String, Box<dyn std::error::Error>> {
    let mut params: HashMap<String, String> = HashMap::new();
    params.insert("exchange".to_owned(), "SSE".to_owned());
    params.insert("start_date".to_owned(), config.data_start_date.to_owned());
    params.insert("end_date".to_owned(), config.data_end_date.to_owned());
    params.insert("is_open".to_owned(), "1".to_owned());
    let api_params = TushareRESTfulAPI {
        api_name: String::from("trade_cal"),
        token: config.tushare_token.to_owned(),
        params: params,
        fields: String::from(""),
    };

    let api_params_json = serde_json::to_string(&api_params).unwrap();
    debug!("{}", api_params_json);

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://api.waditu.com")
        .body(api_params_json)
        .send()?;

    if !res.status().is_success() {
        return Err(Box::new(MyError(String::from(
            "get trade cal res status NOT 200!",
        ))));
    }

    let res_text_str = res.text()?;

    let api_res: serde_json::Value = serde_json::from_str(&res_text_str)?;
    if api_res["code"] != 0 {
        return Err(Box::new(MyError(String::from(format!(
            "get trade cal api return code != 0, ={}, request_id: {} , msg: {}",
            api_res["code"], api_res["request_id"], api_res["msg"]
        )))));
    }

    let api_data = &api_res["data"];
    if api_data["has_more"] != false {
        return Err(Box::new(MyError(String::from("has more?!"))));
    }

    let items = &api_data["items"].as_array().unwrap();

    let mut cal_date_vec: Vec<&str> = Vec::new();
    for i in items.iter() {
        cal_date_vec.push(i[1].as_str().unwrap());
    }
    let latest_trade_date = cal_date_vec.iter().max().unwrap();

    Ok(String::from(latest_trade_date.clone()))
}

/// Data dir
/// --2021-09-01 , dir means lastest hist data date
/// ----hist_data , dir means hist data from start_date to data_date
/// ----stocks_list , file means stocks list on current day
/// _SUCCESS , file means one download finish
fn init_dir(data_dir: &Path, trade_date: String) -> Result<(), Box<dyn Error>> {
    let date_dir = data_dir.join(trade_date);

    debug!("{:?}", date_dir);
    if date_dir.exists() {
        warn!("{:?} exists! rm it", &date_dir);
        fs::remove_dir_all(&date_dir).unwrap();
    }
    fs::create_dir(&date_dir).unwrap();

    let hist_data_dir = date_dir.join("hist_data");
    fs::create_dir(&hist_data_dir).unwrap();

    Ok(())
}

struct StockBasic {
    ts_code: String,
    symbol: String,
    name: String,
    area: String,
    industry: String,
    fullname: String,
    enname: String,
    cnspell: String,
    market: String,
    exchange: String,
    curr_type: String,
    list_status: String,
    list_date: String,
    delist_date: String,
    is_hs: String,
}

fn get_stock_basic(
    config: &Config,
    exchange: &str,
    market: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut params: HashMap<String, String> = HashMap::new();
    params.insert("exchange".to_owned(), exchange.to_owned());
    params.insert("market".to_owned(), market.to_owned());
    params.insert("list_status".to_owned(), "L".to_owned());
    let api_params = TushareRESTfulAPI {
        api_name: String::from("stock_basic"),
        token: config.tushare_token.to_owned(),
        params: params,
        fields: String::from("ts_code, symbol, name, area, industry, fullname, enname, cnspell, market, exchange, curr_type, list_status, list_date, delist_date, is_hs"),
    };

    let api_params_json = serde_json::to_string(&api_params).unwrap();
    debug!("{}", api_params_json);

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://api.waditu.com")
        .body(api_params_json)
        .send()?;

    if !res.status().is_success() {
        return Err(Box::new(MyError(String::from(
            "get trade cal res status NOT 200!",
        ))));
    }

    let res_text_str = res.text()?;
    debug!("{}", res_text_str);

    let api_res: serde_json::Value = serde_json::from_str(&res_text_str)?;
    if api_res["code"] != 0 {
        return Err(Box::new(MyError(String::from(format!(
            "get trade cal return code != 0, ={}, request_id: {} , msg: {}",
            api_res["code"], api_res["request_id"], api_res["msg"]
        )))));
    }

    let api_data = &api_res["data"];
    if api_data["has_more"] != false {
        return Err(Box::new(MyError(String::from("has more?!"))));
    }

    let items = &api_data["items"].as_array().unwrap();
    let mut stocks_base_vec: Vec<&str> = Vec::new();
    for i in items.iter() {
        stocks_base_vec.push(i[0].as_str().unwrap());
        stocks_base_vec.push(i[1].as_str().unwrap());
        stocks_base_vec.push(i[2].as_str().unwrap());
        stocks_base_vec.push(i[3].as_str().unwrap());
        stocks_base_vec.push(i[4].as_str().unwrap());
        stocks_base_vec.push(i[5].as_str().unwrap());
        stocks_base_vec.push(i[6].as_str().unwrap());
        stocks_base_vec.push(i[7].as_str().unwrap());
        stocks_base_vec.push(i[8].as_str().unwrap());
        stocks_base_vec.push(i[9].as_str().unwrap());
        stocks_base_vec.push(i[10].as_str().unwrap());
        stocks_base_vec.push(i[11].as_str().unwrap());
        stocks_base_vec.push(i[12].as_str().unwrap());
        stocks_base_vec.push(i[13].as_str().unwrap_or(None));
        stocks_base_vec.push(i[14].as_str().unwrap());
    }

    println!("{:?}", stocks_base_vec[0]);

    Ok("".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Opt;

    #[test]
    #[ignore]
    fn test_get_latest_trade_cal() {
        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: String::from("20210912"),
        };
        let config = Config::new(args).unwrap();
        assert_eq!(get_latest_trade_cal(&config).unwrap(), "20210910");
    }

    #[test]
    #[ignore]
    fn test_init_dir() {
        env_logger::init();
        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: String::from("20210912"),
        };
        let config = Config::new(args).unwrap();

        assert_eq!(
            init_dir(Path::new(&config.data_dir), "20990101".to_owned()).unwrap(),
            ()
        );
    }

    #[test]
    #[ignore]
    fn test_run() {
        env_logger::init();
        use chrono::offset::Local;

        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: Local::now().format("%Y%m%d").to_string(),
        };
        let config = Config::new(args).unwrap();

        assert_eq!(run(&config).unwrap(), ());
    }

    #[test]
    fn test_get_stock_basic() {
        env_logger::init();
        use chrono::offset::Local;

        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: Local::now().format("%Y%m%d").to_string(),
        };
        let config = Config::new(args).unwrap();

        assert_eq!(
            get_stock_basic(&config, "SSE", "主板").unwrap(),
            "".to_owned()
        );
    }
}
