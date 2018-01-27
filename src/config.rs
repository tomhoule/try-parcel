#[derive(Deserialize, Configure)]
#[serde(default)]

pub struct Config {
    database_url: String,
}

impl Config {
    pub fn database_string(&self) -> &str {
        &self.database_url
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            database_url: "".to_string(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::env::set_var;
//     use configure::Configure;

//     #[test]
//     fn config_extract_database_url() {
//         use_default_config!();
//         let mut c = Config::generate().unwrap();
//         set_var("YACCHAUYO_DATABASE_URL", "abcd");
//         Config::regenerate(&mut c).ok() ;
//         assert_eq!(c.database_url, "abcd")
//     }
// }
