use serde_json::{from_str, to_vec_pretty, to_vec, to_string};
use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::Path;

use crate::data_structures::Conf;
const CONFIG_FILENAME: &str = ".config.json";
const BASE_CONFIG: Conf = Conf {
  host: "<insert_your_host_here>",
  user: "<insert_db_user_here>",
  password: "<insert_db_passwd_here>",
  db: "<insert_db_here>"
};

pub fn create_config_ifn_exists(){
  let p_config = Path::new(CONFIG_FILENAME);
  let mut f_config = File::create(p_config).unwrap();
  // also writes the configurations file because why not?
  let bconfig_j = serde_json::to_string(&BASE_CONFIG).unwrap();
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

pub fn to_config(conf: &str) -> Conf{
  from_str(&conf).unwrap()
}

pub fn get_config_from_config() -> String{
  // this function is a major problem for my tiny brain
  // it's sat. 4 am, I was trying to understand what should I do
  // to make rust stop complaing about borrowing stuff and just giving me the
  // struct I've created, but no, borrowing errors, you know what, rust wins
  // this function only returns the file content now I'm pissed
  let p_config = Path::new(CONFIG_FILENAME);
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
      let e  = BASE_CONFIG;
      to_string(&e).unwrap()
    }
  }
}

