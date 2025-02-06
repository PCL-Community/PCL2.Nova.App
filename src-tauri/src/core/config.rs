use std::{fs::OpenOptions, io::{Read, Write}, path::PathBuf};

thread_local! {
    pub static CONFIG_GLOBAL: Config = Config::new(dirs_next::config_dir().unwrap_or(PathBuf::new()).join("PCL2Nova").to_str().unwrap().to_string(), ConfigType::GLOBAL);
}

pub enum ConfigType {
    GLOBAL, LOCAL
}

pub struct Config {
    config_path: String,
    config_type: ConfigType
}

impl Config {
    pub const fn new(config_path: String, config_type: ConfigType) -> Self {
        Config { config_path, config_type }
    }
    pub fn get_key(&self, key: String) -> Result<String, String> {
        let file = OpenOptions::new()
            .read(true)
            .create(false)
            .open(self.config_path.clone());
        if file.is_err() {
           return Err("File reader is f**ked up.".to_string());
        }
        let mut buf: String = String::new();
        let result = file.unwrap().read_to_string(&mut buf);
        if result.is_err() {
            return Err("JSON parser is f**ed up".to_string());
        }
        let json: serde_json::Value = serde_json::from_str(&buf).unwrap_or(serde_json::Value::Null);
        Ok(json[key].as_str().unwrap_or("").to_string())
    }

    pub fn set_key(&self, key: String, value: serde_json::Value) -> Result<(), String> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .open(self.config_path.clone());
        if file.is_err() {
           return Err("File reader is f**ked up.".to_string());
        }
        let mut buf: String = String::new();
        let read_result = file.as_ref().unwrap().read_to_string(&mut buf);
        if read_result.is_err() {
            return Err("JSON parser is f**ed up".to_string());
        }
        let mut json: serde_json::Value = serde_json::from_str(&buf).unwrap_or(serde_json::Value::Null);
        json[key] = value;
        let write_result = file.as_ref().unwrap().write_all(json.as_str().unwrap().as_bytes());
        if write_result.is_err() {
            return Err("File writer is f**ed up".to_string());
        }
        Ok(())
    }

    pub fn for_each<F: Fn(Option<String>, Option<serde_json::Value>, Option<i32>) -> ()>(&self, opt: F) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .read(true)
            .create(false)
            .open(self.config_path.clone())
            .expect("File reader is f**ked up.");
        let mut buf: String = String::new();
        file.read_to_string(&mut buf).expect("JSON parser is f**ed up");
        let json: serde_json::Value = serde_json::from_str(&buf).unwrap_or(serde_json::Value::Null);
        let values: std::collections::HashMap<String, serde_json::Value> = serde_json::from_value(json).expect("JSON converter f**ked up.");
        let mut i = 0;
        for v in values.iter() {
            opt(Some(v.0.to_string()), Some(v.1.clone()), Some(i));
            i += 1;
        }
        Ok(())
    }

    pub fn override_from_config(&self, config: &Self) -> Result<(), String> {
        config.for_each(|k, v, _| {
            self.set_key(k.unwrap(), v.unwrap()).expect("Config overriding failed.");
        }).expect("Config blows up.");
        Ok(())
    }
}
