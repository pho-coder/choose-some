/// analysis one download data
/// 1. check data
/// 2. init wallet
/// 3. load strategy
/// 4. get one result
use std::error::Error;
use std::path::{Path, PathBuf};

use crate::models::AnalysisResult;
use crate::Config;

pub fn run(config: &Config) -> Result<AnalysisResult, Box<dyn Error>> {
    let data_dir = Path::new(&config.data_dir).join(&config.data_end_date);
    if !check_data(&data_dir) {
        Ok(AnalysisResult {
            finish: false,
            good: true,
        })
    } else {
        Ok(AnalysisResult {
            finish: false,
            good: true,
        })
    }
}

pub fn check_data(date_dir: &PathBuf) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DownloadType, Opt};

    fn get_config() -> Config {
        let args = Opt {
            data_start_date: String::from("20210101"),
            data_end_date: String::from("20210922"),
            download_type: DownloadType::All,
        };
        Config::new(args).unwrap()
    }

    #[test]
    #[ignore]
    fn test_check_data() {}

    #[test]
    #[ignore]
    fn test_run() {
        let config = get_config();
        assert_eq!(run(&config).unwrap().finish, true);
    }
}
