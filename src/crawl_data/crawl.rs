/// Data dir
/// --2021-09-01 , dir means lastest hist data date
/// ----hist_data , dir means hist data from start_date to data_date
/// ----stocks_list , file means stocks list on current day
/// ----_SUCCESS , file means one download finish
use crate::Config;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

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
    let (earliest_trade_date, latest_trade_date) = crawl_trade_cal(config).unwrap();
    let date_dir = data_dir.join(latest_trade_date);

    // init dir
    init_dir(&date_dir)?;

    let token = config.tushare_token.to_owned();
    // get stocks list
    let sse_stocks_basic = crawl_stocks_basic(&token, "SSE", "主板")?;
    let mut szse_stocks_basic = crawl_stocks_basic(&token, "SZSE", "主板")?;
    let mut stocks_basic = sse_stocks_basic;
    stocks_basic.append(&mut szse_stocks_basic);

    // wrtie stocks_list
    let file_name = date_dir.join("stocks_list");
    let file_name_str = file_name.to_str().unwrap();
    write_stocks_list(file_name_str, &stocks_basic)?;

    // read stocks_list
    let stocks_basic = read_stocks_list(file_name_str).unwrap();

    // download stocks basic and write local files
    // download_stocks_daily(token, stocks_basic, earliest_trade_date, latest_trade_date)?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct TushareRESTfulAPI {
    api_name: String,
    token: String,
    params: HashMap<String, String>,
    fields: String,
}

fn crawl_trade_cal(config: &Config) -> Result<(String, String), Box<dyn std::error::Error>> {
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

    let earliest_trade_date = cal_date_vec.iter().min().unwrap();
    let latest_trade_date = cal_date_vec.iter().max().unwrap();

    Ok((
        String::from(earliest_trade_date.clone()),
        String::from(latest_trade_date.clone()),
    ))
}

fn init_dir(date_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
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

#[derive(Debug)]
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
    delist_date: Option<String>,
    is_hs: String,
}

impl StockBasic {
    fn new(a_vec: Vec<String>) -> StockBasic {
        StockBasic {
            ts_code: a_vec[0].clone(),
            symbol: a_vec[1].clone(),
            name: a_vec[2].clone(),
            area: a_vec[3].clone(),
            industry: a_vec[4].clone(),
            fullname: a_vec[5].clone(),
            enname: a_vec[6].clone(),
            cnspell: a_vec[7].clone(),
            market: a_vec[8].clone(),
            exchange: a_vec[9].clone(),
            curr_type: a_vec[10].clone(),
            list_status: a_vec[11].clone(),
            list_date: a_vec[12].clone(),
            delist_date: if a_vec[13] == "null" {
                None
            } else {
                Some(a_vec[13].clone())
            },
            is_hs: a_vec[14].clone(),
        }
    }

    fn to_vec(&self) -> Vec<String> {
        vec![
            self.ts_code.clone(),
            self.symbol.clone(),
            self.name.clone(),
            self.area.clone(),
            self.industry.clone(),
            self.fullname.clone(),
            self.enname.clone(),
            self.cnspell.clone(),
            self.market.clone(),
            self.exchange.clone(),
            self.curr_type.clone(),
            self.list_status.clone(),
            self.list_date.clone(),
            if self.delist_date.is_none() {
                "null".to_owned()
            } else {
                self.delist_date.clone().unwrap()
            },
            self.is_hs.clone(),
        ]
    }

    fn to_string(&self) -> String {
        self.to_vec().join("\t")
    }

    fn string2vec(a_string: String) -> Vec<String> {
        a_string.split("\t").map(|s| s.to_string()).collect()
    }

    fn from_string(a_string: String) -> StockBasic {
        let a_vec = StockBasic::string2vec(a_string);
        StockBasic::new(a_vec)
    }
}

fn crawl_stocks_basic(
    token: &str,
    exchange: &str,
    market: &str,
) -> Result<Vec<StockBasic>, Box<dyn std::error::Error>> {
    let mut params: HashMap<String, String> = HashMap::new();
    params.insert("exchange".to_owned(), exchange.to_owned());
    params.insert("market".to_owned(), market.to_owned());
    params.insert("list_status".to_owned(), "L".to_owned());
    let api_params = TushareRESTfulAPI {
        api_name: String::from("stock_basic"),
        token: token.to_owned(),
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
    let mut stocks_base_vec: Vec<StockBasic> = Vec::new();
    for i in items.iter() {
        let stock_basic = StockBasic {
            ts_code: i[0].as_str().unwrap().to_owned(),
            symbol: i[1].as_str().unwrap().to_owned(),
            name: i[2].as_str().unwrap().to_owned(),
            area: i[3].as_str().unwrap().to_owned(),
            industry: i[4].as_str().unwrap().to_owned(),
            fullname: i[5].as_str().unwrap().to_owned(),
            enname: i[6].as_str().unwrap().to_owned(),
            cnspell: i[7].as_str().unwrap().to_owned(),
            market: i[8].as_str().unwrap().to_owned(),
            exchange: i[9].as_str().unwrap().to_owned(),
            curr_type: i[10].as_str().unwrap().to_owned(),
            list_status: i[11].as_str().unwrap().to_owned(),
            list_date: i[12].as_str().unwrap().to_owned(),
            delist_date: if i[13].is_null() {
                None
            } else {
                Some(i[13].as_str().unwrap().to_owned())
            },
            is_hs: i[14].as_str().unwrap().to_owned(),
        };
        stocks_base_vec.push(stock_basic);
    }

    debug!("{:?}", stocks_base_vec[0]);

    Ok(stocks_base_vec)
}

fn write_stocks_list(
    file_name: &str,
    stocks_basic_vec: &Vec<StockBasic>,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("{}", stocks_basic_vec.len());

    let mut file = fs::File::create(file_name).unwrap();
    write!(&mut file, "ts_code\tsymbol\tname\tarea\tindustry\tfullname\tenname\tcnspell\tmarket\texchange\tcurr_type\tlist_status\tlist_date\tdelist_date\tis_hs\n").unwrap();
    for stock_basic in stocks_basic_vec {
        let stock_basic_string = stock_basic.to_string();
        write!(&mut file, "{}{}", stock_basic_string, "\n").unwrap();
    }

    Ok(())
}

fn read_stocks_list(file_name: &str) -> Result<Vec<StockBasic>, MyError> {
    let file = fs::File::open(file_name).unwrap();
    let file_buffered = BufReader::new(file);
    let mut lines = file_buffered.lines();
    let mut a_vec: Vec<StockBasic> = vec![];
    // skip first line
    lines.next();
    for line in lines {
        a_vec.push(StockBasic::from_string(line.unwrap()));
    }
    debug!("{:?}", a_vec[0]);
    debug!("{:?}", a_vec[a_vec.len() - 1]);
    Ok(a_vec)
}

// max crawl months is 23, if want to crawl 10 codes everytime.
fn download_stocks_daily(
    token: &str,
    stocks_basic: Vec<StockBasic>,
    start_date: &str,
    end_date: &str,
) {
    info!("will download {} stocks daily", stocks_basic.len());
    let MAX_CODES = 10;
    let mut ts_code_grouped: Vec<Vec<String>> = vec![];
    let mut current_ts_codes_grouped: Vec<String> = vec![];
    for stock_basic in stocks_basic {
        if ts_code_grouped.len() == MAX_CODES {
            ts_code_grouped.push(current_ts_codes_grouped);
            current_ts_codes_grouped = vec![];
            current_ts_codes_grouped.push(stock_basic.ts_code);
        } else {
            current_ts_codes_grouped.push(stock_basic.ts_code);
        }
    }
    if !current_ts_codes_grouped.is_empty() {
        ts_code_grouped.push(current_ts_codes_grouped);
    }

    for ts_codes_group in ts_code_grouped {
        let stocks_daily_vec =
            crawl_stocks_daily(token, ts_codes_group, start_date, end_date).unwrap();
    }
}

#[derive(Debug)]
struct StockDaily {
    ts_code: String,
    trade_date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    pre_close: f64,
    change: f64,
    pct_chg: f64,
    vol: f64,
    amount: f64,
}

// 每分钟内最多调取500次，每次5000条数据. so max crawl months is 23, if want to crawl 10 codes everytime.
fn crawl_stocks_daily(
    token: &str,
    ts_codes: Vec<String>,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<StockDaily>, Box<dyn std::error::Error>> {
    let mut params: HashMap<String, String> = HashMap::new();
    params.insert("ts_code".to_owned(), ts_codes.join(",").to_owned());
    params.insert("start_date".to_owned(), start_date.to_owned());
    params.insert("end_date".to_owned(), end_date.to_owned());
    let api_params = TushareRESTfulAPI {
        api_name: String::from("daily"),
        token: token.to_owned(),
        params: params,
        fields: String::from(
            "ts_code, trade_date, open, high, low, close, pre_close, change, pct_chg, vol, amount",
        ),
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
    // debug!("{}", api_res);

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
    let mut stocks_daily_vec: Vec<StockDaily> = Vec::new();
    for i in items.iter() {
        let stock_daily = StockDaily {
            ts_code: i[0].as_str().unwrap().to_owned(),
            trade_date: i[1].as_str().unwrap().to_owned(),
            open: i[2].as_f64().unwrap().to_owned(),
            high: i[3].as_f64().unwrap().to_owned(),
            low: i[4].as_f64().unwrap().to_owned(),
            close: i[5].as_f64().unwrap().to_owned(),
            pre_close: i[6].as_f64().unwrap().to_owned(),
            change: i[7].as_f64().unwrap().to_owned(),
            pct_chg: i[8].as_f64().unwrap().to_owned(),
            vol: i[9].as_f64().unwrap().to_owned(),
            amount: i[10].as_f64().unwrap().to_owned(),
        };
        stocks_daily_vec.push(stock_daily);
    }

    debug!("{:?}", stocks_daily_vec[0]);

    Ok(stocks_daily_vec)
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
        assert_eq!(crawl_trade_cal(&config).unwrap().1, "20210910");
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
        let date_dir = Path::new(&config.data_dir).join("20990101".to_owned());

        assert_eq!(init_dir(&date_dir).unwrap(), ());
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
    #[ignore]
    fn test_get_stock_basic() {
        env_logger::init();
        use chrono::offset::Local;

        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: Local::now().format("%Y%m%d").to_string(),
        };
        let config = Config::new(args).unwrap();
        let token = config.tushare_token;

        let result = crawl_stocks_basic(&token, "SSE", "主板").unwrap();
        let result_len = result.len();
        println!("{}", result_len);
        assert!(result_len >= 1);
    }

    #[test]
    #[ignore]
    fn test_write_stocks_basic() {
        env_logger::init();
        use chrono::offset::Local;

        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: Local::now().format("%Y%m%d").to_string(),
        };
        let config = &Config::new(args).unwrap();
        let token = config.tushare_token.clone();

        let data_dir = Path::new(&config.data_dir);
        let trade_date = crawl_trade_cal(config).unwrap().1;
        let date_dir = data_dir.join(trade_date);

        // init dir
        init_dir(&date_dir).unwrap();

        // wrtie stocks_list
        let file_name = date_dir.join("stocks_list");

        let stocks_basic_vec = crawl_stocks_basic(&token, "SSE", "主板").unwrap();
        let result = write_stocks_list(file_name.to_str().unwrap(), &stocks_basic_vec).unwrap();
        assert_eq!(result, ());
    }

    #[test]
    #[ignore]
    fn test_read_stocks_list() {
        env_logger::init();
        let file_name = "/Users/phoenix/data/20210917/stocks_list";
        assert!(read_stocks_list(file_name).unwrap().len() > 1);
    }

    #[test]
    fn test_crawl_stocks_daily() {
        env_logger::init();
        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: String::from("20210901"),
        };
        let config = &Config::new(args).unwrap();
        let token = config.tushare_token.clone();
        let ts_codes = vec!["689009.SH".to_owned(), "688981.SH".to_owned()];
        let start_date = "20210901";
        let end_date = "20210910";

        let stocks_daily_vec = crawl_stocks_daily(&token, ts_codes, start_date, end_date);
        assert!(stocks_daily_vec.unwrap().len() > 1);
    }
}
