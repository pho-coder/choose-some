/// Data dir
/// --2021-09-01 , dir means lastest hist data date
/// ----daily_data , dir means hist data from start_date to data_date
/// ----stocks_list , file means stocks list on current day
/// ----_SUCCESS , file means one download finish
use crate::Config;
use crate::DownloadType;
use log::{debug, info, warn};
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

use crate::models::{StockBasic, StockDaily, StockDailyBasic, TushareRESTfulAPI};

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
    let date_dir = data_dir.join(&latest_trade_date);

    // init dir
    init_dir(&date_dir)?;

    let token = config.tushare_token.to_owned();
    // get stocks list
    let sse_stocks_basic = crawl_stocks_basic(&token, "SSE", "主板")?;
    let mut szse_stocks_basic = crawl_stocks_basic(&token, "SZSE", "主板")?;
    let mut stocks_basic = sse_stocks_basic;
    stocks_basic.append(&mut szse_stocks_basic);

    // wrtie stocks_list
    let stocks_list_file_name = date_dir.join("stocks_list");
    write_stocks_list(&stocks_list_file_name, &stocks_basic)?;

    // read stocks_list
    let stocks_basic = read_stocks_list(&stocks_list_file_name).unwrap();

    // download stocks daily and basic and write local files
    download_stocks_daily(
        &date_dir,
        &token,
        &stocks_basic,
        &earliest_trade_date,
        &latest_trade_date,
        config.download_type,
    )?;

    Ok(())
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

    let daily_data_dir = date_dir.join("daily_data");
    fs::create_dir(&daily_data_dir).unwrap();

    let daily_basic_data_dir = date_dir.join("daily_basic_data");
    fs::create_dir(&daily_basic_data_dir).unwrap();

    Ok(())
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
    file_name: &PathBuf,
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

fn read_stocks_list(file_name: &PathBuf) -> Result<Vec<StockBasic>, MyError> {
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
    date_dir: &PathBuf,
    token: &str,
    stocks_basic: &Vec<StockBasic>,
    start_date: &str,
    end_date: &str,
    download_type: DownloadType,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("will download {} stocks daily", stocks_basic.len());
    let max_codes = 10;
    let mut ts_code_grouped: Vec<Vec<String>> = vec![];
    let mut current_ts_codes_grouped: Vec<String> = vec![];
    for stock_basic in stocks_basic {
        if current_ts_codes_grouped.len() == max_codes {
            ts_code_grouped.push(current_ts_codes_grouped);
            current_ts_codes_grouped = vec![];
            current_ts_codes_grouped.push(stock_basic.ts_code.clone());
        } else {
            current_ts_codes_grouped.push(stock_basic.ts_code.clone());
        }
    }
    if !current_ts_codes_grouped.is_empty() {
        ts_code_grouped.push(current_ts_codes_grouped);
    }

    if download_type == DownloadType::All || download_type == DownloadType::Daily {
        let daily_data_dir = date_dir.join("daily_data");
        for ts_codes_group in ts_code_grouped.clone() {
            let stocks_daily_vec =
                crawl_stocks_daily(token, ts_codes_group.clone(), start_date, end_date).unwrap();
            for ts_code in ts_codes_group {
                let file_name = daily_data_dir.join(&ts_code);
                debug!("{:?}", file_name);
                let one_stock_daily: Vec<StockDaily> = stocks_daily_vec
                    .iter()
                    .filter(|s| s.ts_code == ts_code.to_string())
                    .cloned()
                    .collect();
                // write one stock daily data
                let mut file = fs::File::create(file_name).unwrap();
                write!(&mut file, "ts_code\ttrade_date\topen\thigh\tlow\tclose\tpre_close\tchange\tpct_chg\tvol\tamount\n").unwrap();
                for stock_daily in one_stock_daily {
                    let stock_daily_string = stock_daily.to_string();
                    write!(&mut file, "{}{}", stock_daily_string, "\n").unwrap();
                }
            }
        }
    }

    if download_type == DownloadType::All || download_type == DownloadType::DailyBasic {
        let daily_basic_data_dir = date_dir.join("daily_basic_data");
        for ts_codes_group in ts_code_grouped.clone() {
            let stocks_daily_basic_vec =
                crawl_stocks_daily_basic(token, ts_codes_group.clone(), start_date, end_date)
                    .unwrap();
            for ts_code in ts_codes_group {
                let file_name = daily_basic_data_dir.join(&ts_code);
                debug!("{:?}", file_name);
                let one_stock_basic_daily: Vec<StockDailyBasic> = stocks_daily_basic_vec
                    .iter()
                    .filter(|s| s.ts_code == ts_code.to_string())
                    .cloned()
                    .collect();
                // write one stock daily basic data
                let mut file = fs::File::create(file_name).unwrap();
                write!(&mut file, "ts_code\ttrade_date\tclose\tturnover_rate\tturnover_rate_f\tvolume_ratio\tpe\tpe_ttm\tpb\tps\tps_ttm\tdv_ratio\tdv_ttm\ttotal_share\tfloat_share\tfree_share\ttotal_mv\tcirc_mv\tlimit_status\n").unwrap();
                for stock_daily_basic in one_stock_basic_daily {
                    let stock_daily_basic_string = stock_daily_basic.to_string();
                    write!(&mut file, "{}{}", stock_daily_basic_string, "\n").unwrap();
                }
            }
        }
    }

    // write finish file _SUCCESS
    let mut file = fs::File::create(date_dir.join("_SUCCESS")).unwrap();
    let mut result_vec: Vec<&str> = vec![];
    if download_type == DownloadType::All {
        result_vec = vec!["daily", "daily_basic"];
    } else if download_type == DownloadType::Daily {
        result_vec = vec!["daily"];
    } else if download_type == DownloadType::Daily {
        result_vec = vec!["daily_basic"];
    }
    let result_str = result_vec.join("\n");
    file.write_all(result_str.as_bytes()).unwrap();

    Ok(())
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

fn crawl_stocks_daily_basic(
    token: &str,
    ts_codes: Vec<String>,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<StockDailyBasic>, Box<dyn std::error::Error>> {
    let mut params: HashMap<String, String> = HashMap::new();
    params.insert("ts_code".to_owned(), ts_codes.join(",").to_owned());
    params.insert("start_date".to_owned(), start_date.to_owned());
    params.insert("end_date".to_owned(), end_date.to_owned());
    let api_params = TushareRESTfulAPI {
        api_name: String::from("daily_basic"),
        token: token.to_owned(),
        params: params,
        fields: String::from(
            "ts_code, trade_date, close, turnover_rate, turnover_rate_f, volume_ratio, pe, pe_ttm, pb, ps, ps_ttm, dv_ratio, dv_ttm, total_share, float_share, free_share, total_mv, circ_mv, limit_status",
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
    let mut stocks_daily_basic_vec: Vec<StockDailyBasic> = Vec::new();
    for i in items.iter() {
        debug!("{:?}", i);
        let stock_daily_basic = StockDailyBasic {
            ts_code: i[0].as_str().unwrap().to_owned(),
            trade_date: i[1].as_str().unwrap().to_owned(),
            close: i[2].as_f64().unwrap().to_owned(),
            turnover_rate: i[3].as_f64().unwrap().to_owned(),
            turnover_rate_f: if i[4].is_null() {
                None
            } else {
                Some(i[4].as_f64().unwrap().to_owned())
            },
            volume_ratio: if i[5].is_null() {
                None
            } else {
                Some(i[5].as_f64().unwrap().to_owned())
            },
            pe: if i[6].is_null() {
                None
            } else {
                Some(i[6].as_f64().unwrap().to_owned())
            },
            pe_ttm: if i[7].is_null() {
                None
            } else {
                Some(i[7].as_f64().unwrap().to_owned())
            },
            pb: if i[8].is_null() {
                None
            } else {
                Some(i[8].as_f64().unwrap().to_owned())
            },
            ps: if i[9].is_null() {
                None
            } else {
                Some(i[9].as_f64().unwrap().to_owned())
            },
            ps_ttm: if i[10].is_null() {
                None
            } else {
                Some(i[10].as_f64().unwrap().to_owned())
            },
            dv_ratio: if i[11].is_null() {
                None
            } else {
                Some(i[11].as_f64().unwrap().to_owned())
            },
            dv_ttm: if i[12].is_null() {
                None
            } else {
                Some(i[12].as_f64().unwrap().to_owned())
            },
            total_share: i[13].as_f64().unwrap().to_owned(),
            float_share: i[14].as_f64().unwrap().to_owned(),
            free_share: i[15].as_f64().unwrap().to_owned(),
            total_mv: i[16].as_f64().unwrap().to_owned(),
            circ_mv: i[17].as_f64().unwrap().to_owned(),
            limit_status: if i[18].is_null() {
                None
            } else {
                Some(i[18].as_i64().unwrap().to_owned())
            },
        };
        stocks_daily_basic_vec.push(stock_daily_basic);
    }

    debug!("{:?}", stocks_daily_basic_vec[0]);

    Ok(stocks_daily_basic_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DownloadType, Opt};

    #[test]
    #[ignore]
    fn test_get_latest_trade_cal() {
        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: String::from("20210912"),
            download_type: DownloadType::All,
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
            download_type: DownloadType::All,
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
            download_type: DownloadType::All,
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
            download_type: DownloadType::All,
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
            download_type: DownloadType::All,
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
        let result = write_stocks_list(&file_name, &stocks_basic_vec).unwrap();
        assert_eq!(result, ());
    }

    #[test]
    #[ignore]
    fn test_read_stocks_list() {
        env_logger::init();
        let file_name = PathBuf::from("/Users/phoenix/data/20210917/stocks_list");
        assert!(read_stocks_list(&file_name).unwrap().len() > 1);
    }

    #[test]
    fn test_crawl_stocks_daily() {
        env_logger::init();
        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: String::from("20210901"),
            download_type: DownloadType::All,
        };
        let config = &Config::new(args).unwrap();
        let token = config.tushare_token.clone();
        let ts_codes = vec!["689009.SH".to_owned(), "688981.SH".to_owned()];
        let start_date = "20210901";
        let end_date = "20210910";

        let stocks_daily_vec = crawl_stocks_daily(&token, ts_codes, start_date, end_date);
        assert!(stocks_daily_vec.unwrap().len() > 1);
    }

    #[test]
    fn test_crawl_stocks_daily_basic() {
        env_logger::init();
        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: String::from("20210901"),
            download_type: DownloadType::All,
        };
        let config = &Config::new(args).unwrap();
        let token = config.tushare_token.clone();
        let ts_codes = vec!["689009.SH".to_owned(), "688981.SH".to_owned()];
        let start_date = "20210901";
        let end_date = "20210910";

        let stocks_daily_basic_vec =
            crawl_stocks_daily_basic(&token, ts_codes, start_date, end_date);
        assert!(stocks_daily_basic_vec.unwrap().len() > 1);
    }

    #[test]
    fn test_download_stocks_daily() {
        env_logger::init();
        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: String::from("20210917"),
            download_type: DownloadType::All,
        };
        let config = &Config::new(args).unwrap();
        let token = config.tushare_token.clone();
        let start_date = "20210901";
        let end_date = "20210917";
        let date_dir = PathBuf::from(config.data_dir.clone() + "/20210917");

        let file_name = PathBuf::from("/Users/phoenix/data/20210917/stocks_list");
        let stocks_basic = &read_stocks_list(&file_name).unwrap()[..10].to_vec();

        assert_eq!(
            download_stocks_daily(
                &date_dir,
                &token,
                stocks_basic,
                start_date,
                end_date,
                DownloadType::All,
            )
            .unwrap(),
            ()
        );
    }
}
