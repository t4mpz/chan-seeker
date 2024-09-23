use serde_json::{from_str, to_string};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::data_structures::Conf;
const CONFIG_FILENAME: &str = ".config.json";

fn gen_base_config() -> Conf{
  // had to move the constant to a function because of the string values
  // it was working before but it was very ugly
  Conf {
    host: "<insert_your_host_here>".to_string(),
    user: "<insert_db_user_here>".to_string(),
    password: "<insert_db_passwd_here>".to_string(),
    db: "<insert_db_here>".to_string()
  }
}
macro_rules! p_config {
    () => {
      Path::new(CONFIG_FILENAME)  
    };
}

pub fn create_config_ifn_exists(){
  let p_config = p_config!();
  let mut f_config = File::create(p_config).unwrap();
  // also writes the configurations file because why not?
  let bconfig_j = serde_json::to_string(&gen_base_config()).unwrap();
  let r_write = f_config.write_all(bconfig_j.as_bytes());
  match r_write{
    Err(why) => {
      println!("Couldn't write base config. {}", why.to_string());
    }
    Ok(_) => {
      println!("Wrote the base config to the file");
    },
  }
}


fn read_config() -> String{
  let p_config = p_config!();
  let f_config = File::open(p_config);
  match f_config{
    Ok(mut config) => {
      let mut config_s = String::new();
      config.read_to_string(&mut config_s).unwrap();
      config_s
    }
    Err(_err) => {
      // presumes that's a file doesn't exists error
      println!("got an {}", _err.to_string());
      create_config_ifn_exists();
      let e  = gen_base_config();
      to_string(&e).unwrap()
    }
  }
}

pub fn get_config() -> Conf{
  let p_config = p_config!();
  match p_config.exists(){
    true => {
      from_str(&read_config()).unwrap()
    }
    false => {
      create_config_ifn_exists();
      gen_base_config()
    }
  }
}

pub fn save_config(conf: Conf){
  let p_config = p_config!();
  let n_config = serde_json::to_string(&conf).unwrap();
  let f_conf = File::create(p_config);
  match f_conf{
    Ok(mut fl) => {
      fl.write_all(n_config.as_bytes()).unwrap();
      println!("Saved new content");
    }
    Err(_) => {
      println!("Probably the file doesn't exists");
      create_config_ifn_exists();
      save_config(conf);
    }
  }
}

