use crate::Config;
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

fn _test_type<T>(_: T) {
    println!("{:?}", { type_name::<T>() });
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{} {}", config.data_start_date, config.data_end_date);
    init_dir();

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct TradeCalParams {
    api_name: String,
    token: String,
    params: HashMap<String, String>,
    fields: String,
}

#[derive(Debug)]
struct MyError(String);
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for MyError {}

fn get_trade_cal(config: Config) -> Result<String, Box<dyn std::error::Error>> {
    let mut params = HashMap::new();
    params.insert("exchange".to_owned(), "SSE".to_owned());
    params.insert("start_date".to_owned(), config.data_start_date);
    params.insert("end_date".to_owned(), config.data_end_date);
    params.insert("is_open".to_owned(), "1".to_owned());
    let trade_cal_params = TradeCalParams {
        api_name: String::from("trade_cal"),
        token: config.tushare_token,
        params: params,
        fields: String::from(""),
    };

    let params_json = serde_json::to_string(&trade_cal_params).unwrap();
    // println!("{}", params_json);

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://api.waditu.com")
        .body(params_json)
        .send()?;

    if !res.status().is_success() {
        return Err(Box::new(MyError(String::from(
            "get trade cal res status NOT 200!",
        ))));
    }

    let res_text_str = res.text()?;

    let trade_cal_res: serde_json::Value = serde_json::from_str(&res_text_str)?;
    if trade_cal_res["code"] != 0 {
        return Err(Box::new(MyError(String::from(format!(
            "get trade cal return code != 0, ={}, request_id: {} , msg: {}",
            trade_cal_res["code"], trade_cal_res["request_id"], trade_cal_res["msg"]
        )))));
    }

    let trade_cal_data = &trade_cal_res["data"];
    if trade_cal_data["has_more"] != false {
        return Err(Box::new(MyError(String::from("has more?!"))));
    }

    let items = &trade_cal_data["items"].as_array().unwrap();

    let mut cal_date_vec: Vec<&str> = Vec::new();
    for i in items.iter() {
        cal_date_vec.push(i[1].as_str().unwrap());
    }
    let max_trade_date = cal_date_vec.iter().max().unwrap();

    Ok(String::from(max_trade_date.clone()))
}

// fn cal_trade_cal() -> Result<(), Box<dyn Error>> {}

/// Data dir
/// --2021-09-01 , means lastest hist data date
/// ----hist_data , means hist data from start_date to data_date
/// ----stocks_list , means stocks list on current day
/// _SUCCESS , file means one download finish
fn init_dir(trade_date: String) -> Result<(), Box<dyn Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Opt;

    #[test]
    fn test_get_trade_cal() {
        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: String::from("20210912"),
        };
        let config = Config::new(args).unwrap();
        assert_eq!(get_trade_cal(config).unwrap(), "20210910");
    }
}
