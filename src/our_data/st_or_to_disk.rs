use crate::fsio::{self, mkdir};
use crate::log::{err, info, log, warn};
use crate::{path_exists, step, ST};

// fn to save our st struct to disk
pub fn store_st(s: &ST) -> () {
    let cfg_dir = "var/uuid";

    let st_json = match serde_json::to_string(&s) {
        Ok(j) => j,
        Err(e) => panic!("[p] unable to serialise ST: {:?}", e),
    };
    info("Saving ST to disk");
    let st_serde_filename = format!("{}/{}", &cfg_dir, "ST.serde");
    log(format!("storing ST to disk: {:?}", &s.show()));
    if !path_exists(&cfg_dir) {
        let should_exist = mkdir(&cfg_dir);
        if !path_exists(&cfg_dir) {
            panic!("[e] var dir {:?}", should_exist);
        }
    }
    if path_exists(&cfg_dir) {
        match fsio::write(&st_serde_filename, &st_json) {
            Err(e) => {
                err(format!("Failed to store ST: {:#?}", e));
                panic!("You should fix this");
            }
            _ => (),
        };
    } else {
        warn(format!("var dir {:?} missing", &cfg_dir));
    }
}

pub fn fetch_st() -> ST {
    let cfg_dir = "var/uuid";
    let st_serde_filename = format!("{}/{}", &cfg_dir, "ST.serde");
    if !path_exists(&cfg_dir) {
        let var_dir_created = mkdir(&cfg_dir);
        println!("{:?}", var_dir_created);
    }

    if path_exists(&st_serde_filename) {
        match fsio::read(&st_serde_filename) {
            Err(e) => {
                err(format!("Failed to read ST from disk: {:#?}", e));
                panic!("You should fix this");
            }
            Ok(json) => {
                use serde::Deserialize;
                use serde_json::Value;

                let mut deserializer = serde_json::Deserializer::from_str(&json);
                deserializer.disable_recursion_limit();
                let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
                let value = Value::deserialize(deserializer).unwrap();

                return serde_json::from_value(value).unwrap();
            }
        };
    } else {
        info(format!(
            "ST.serde {:?} not found (Is this a first run?)",
            &cfg_dir
        ));
        step();
    }
    // generate a new ST if we didn't find a valid one on disk
    ST::new()
}
