use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct TushareRESTfulAPI {
    pub api_name: String,
    pub token: String,
    pub params: HashMap<String, String>,
    pub fields: String,
}

#[derive(Debug, Clone)]
pub struct StockBasic {
    pub ts_code: String,
    pub symbol: String,
    pub name: String,
    pub area: String,
    pub industry: String,
    pub fullname: String,
    pub enname: String,
    pub cnspell: String,
    pub market: String,
    pub exchange: String,
    pub curr_type: String,
    pub list_status: String,
    pub list_date: String,
    pub delist_date: Option<String>,
    pub is_hs: String,
}

impl StockBasic {
    // from local file, not http
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
            delist_date: if a_vec[13] == "none" {
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
                "none".to_owned()
            } else {
                self.delist_date.clone().unwrap()
            },
            self.is_hs.clone(),
        ]
    }

    pub fn to_string(&self) -> String {
        self.to_vec().join("\t")
    }

    fn string2vec(a_string: String) -> Vec<String> {
        a_string.split("\t").map(|s| s.to_string()).collect()
    }

    pub fn from_string(a_string: String) -> StockBasic {
        let a_vec = StockBasic::string2vec(a_string);
        StockBasic::new(a_vec)
    }
}

#[derive(Debug, Clone)]
pub struct StockDaily {
    pub ts_code: String,
    pub trade_date: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub pre_close: f64,
    pub change: f64,
    pub pct_chg: f64,
    pub vol: f64,
    pub amount: f64,
}

impl StockDaily {
    fn to_vec(&self) -> Vec<String> {
        vec![
            String::from(self.ts_code.clone()),
            String::from(self.trade_date.clone()),
            self.open.to_string(),
            self.high.to_string(),
            self.low.to_string(),
            self.close.to_string(),
            self.pre_close.to_string(),
            self.change.to_string(),
            self.pct_chg.to_string(),
            self.vol.to_string(),
            self.amount.to_string(),
        ]
    }

    pub fn to_string(&self) -> String {
        self.to_vec().join("\t")
    }
}

#[derive(Debug, Clone)]
pub struct StockDailyBasic {
    pub ts_code: String,
    pub trade_date: String,
    pub close: f64,
    pub turnover_rate: f64,
    pub turnover_rate_f: f64,
    pub volume_ratio: Option<f64>,
    pub pe: Option<f64>,
    pub pe_ttm: Option<f64>,
    pub pb: f64,
    pub ps: f64,
    pub ps_ttm: f64,
    pub dv_ratio: Option<f64>,
    pub dv_ttm: Option<f64>,
    pub total_share: f64,
    pub float_share: f64,
    pub free_share: f64,
    pub total_mv: f64,
    pub circ_mv: f64,
    pub limit_status: Option<i64>,
}

impl StockDailyBasic {
    fn to_vec(&self) -> Vec<String> {
        vec![
            String::from(self.ts_code.clone()),
            String::from(self.trade_date.clone()),
            self.close.to_string(),
            self.turnover_rate.to_string(),
            self.turnover_rate_f.to_string(),
            if self.volume_ratio.is_none() {
                "none".to_owned()
            } else {
                self.volume_ratio.unwrap().to_string()
            },
            if self.pe.is_none() {
                "none".to_owned()
            } else {
                self.pe.unwrap().to_string()
            },
            if self.pe_ttm.is_none() {
                "none".to_owned()
            } else {
                self.pe_ttm.unwrap().to_string()
            },
            self.pb.to_string(),
            self.ps.to_string(),
            self.ps_ttm.to_string(),
            if self.dv_ratio.is_none() {
                "none".to_owned()
            } else {
                self.dv_ratio.unwrap().to_string()
            },
            if self.dv_ttm.is_none() {
                "none".to_owned()
            } else {
                self.dv_ttm.unwrap().to_string()
            },
            self.total_share.to_string(),
            self.float_share.to_string(),
            self.free_share.to_string(),
            self.total_mv.to_string(),
            self.circ_mv.to_string(),
            if self.limit_status.is_none() {
                "none".to_owned()
            } else {
                self.limit_status.unwrap().to_string()
            },
        ]
    }

    pub fn to_string(&self) -> String {
        self.to_vec().join("\t")
    }
}
