use chrono::{DateTime, Utc};

pub const NIL: &'static str = "\x1b[0m"; // reset/remove colour
pub const RED: &'static str = "\x1b[0;31m"; // red
pub const ERR: &'static str = "\x1b[1;31m"; // error
pub const INF: &'static str = "\x1b[1;32m"; // info
pub const WRN: &'static str = "\x1b[1;33m"; // HLT/warn
pub const LOG: &'static str = "\x1b[1;34m"; // log
pub const MAG: &'static str = "\x1b[1;35m"; // magenta
pub const LGA: &'static str = "\x1b[1;36m"; // Log Aqua
pub const CYN: &'static str = "\x1b[1;36m"; // cyan

pub fn log<T: std::fmt::Display>(msg: T) -> () {
    let dt: DateTime<Utc> = Utc::now();
    #[rustfmt::skip]
    println!("{}[{}{}{}]{} {}{}",MAG,CYN,dt.format("%Y-%m-%d_%H:%M:%S_%Z").to_string(),MAG,LOG,msg,NIL);
}
pub fn warn<T: std::fmt::Display>(msg: T) -> () {
    eprintln!("{}[{}w{}]{} {}{}", INF, LGA, INF, WRN, msg, NIL);
}
pub fn info<T: std::fmt::Display>(msg: T) -> () {
    eprintln!("{}[{}i{}]{} {}{}", WRN, INF, WRN, CYN, msg, NIL);
}
pub fn info_n<T: std::fmt::Display>(msg: T) -> () {
    eprint!("{}[{}i{}]{} {}{}", WRN, INF, WRN, CYN, msg, NIL);
}
pub fn err<T: std::fmt::Display>(msg: T) -> () {
    eprintln!("{}[{}e{}]{} {}{}", RED, ERR, RED, LOG, msg, NIL);
}

use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::default::Default;
use std::error::Error;
use std::fs::{self, File, OpenOptions}; //self: mkdir; File: read; OpenOptions: write
use std::io::{self, Read, Write};
use std::{io::prelude::*, thread, time}; // fn step()

// listen for the kill signals
//use std::thread;
extern crate signal_hook;
use signal_hook::{iterator::Signals, SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use std::ptr::NonNull;

unsafe impl core::marker::Send for ST {}
// evil code just to make last_gasp work
struct Wrapper(NonNull<ST>);
unsafe impl std::marker::Send for Wrapper {}

/// This is triggered by the Signals::
fn last_gasp(sig: i32, st: &ST) -> () {
    println!("Writing current ST to disk");
    store_st(&st);
    // write board_to_html while we are at it
    println!("Files written (phew!); exiting now {:#?}", sig);
    std::process::exit(sig);
}

pub fn write(file: &str, data: &str) -> Result<String, io::Error> {
    let mut fh = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file)?;
    match fh.write_all(data.as_bytes()) {
        Ok(_) => Ok("Ok".to_string()),
        Err(e) => Err(e),
    }
}

pub fn read(f: &str) -> Result<String, io::Error> {
    let f = File::open(f);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn mkdir(path: &str) -> std::io::Result<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

/// SearchTree struct
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ST {
    pub v: Option<(u16, u8)>, //value
    pub c: Option<Vec<ST>>,   // children
}

impl Default for ST {
    fn default() -> Self {
        ST { v: None, c: None }
    }
}

impl ST {
    fn show(&self) {
        &self;
    }

    // let mut st = ST { ..ST::default() };
    pub fn new() -> ST {
        ST { ..ST::default() }
    }

    pub fn add_child(&mut self, child: Vec<ST>) {
        match &mut self.c {
            None => self.c = Some(child),
            Some(cv) => {
                cv.extend(child);
            }
        };
    }

    // used for testing
    #[allow(dead_code)]
    fn add_st(&mut self, child: ST) {
        match &mut self.c {
            None => {
                self.c = Some(vec![child]);
            }
            Some(cv) => {
                cv.push(child);
            }
        };
    }

    pub fn len(&self) -> usize {
        (&self).len_loop(0_usize)
    }

    fn len_loop(&self, depth: usize) -> usize {
        match &self.c {
            Some(c) => {
                for e in c.iter() {
                    e.len_loop(depth + 1);
                }
                depth + 1
            }
            None => 0,
        }
    }
}

#[derive(Debug)]
pub struct STItem(Vec<u16>, Vec<(u16, u8)>); // does this have to be a Vec or can one or both be an array?

impl Iterator for ST {
    //type Item = (Vec<u16>, Vec<(u16, u8)>); //abstract that for use in the other Iterators
    type Item = STItem;
    fn next(&mut self) -> Option<Self::Item> {
        let mut board: Vec<(u16, u8)> = vec![]; //empty board; we skip the root node because it just exists to hold all of the first pieces
        let path: Vec<u16> = match &mut self.c {
            Some(children) => {
                match self.v {
                    // add the current value to the board (if there is one)
                    Some(_value) => board.push(self.v.unwrap()),
                    None => {}
                };
                // iterate children and call next(child) on each of them NTS to be written once we grok recursive struct [iter,&iter_mut,&iter]
                let iter_index = 0;
                let step_on_the_path = match &(*children)[iter_index] {
                    ST {
                        v: Some(_v),
                        c: Some(_c),
                    } => iter_index,
                    //ST {Some(v),Some(c) } => self.v.unwrap(),
                    _ => 0,
                };
                //self.c = Some(children[1..].to_vec());  // cannot assign to `self.c` because it is borrowed
                //self.v = *(children)[0].v;
                self.v = Some((children)[0].v.unwrap());
                vec![step_on_the_path.try_into().unwrap()]
            }
            None => vec![],
        }; // let path:
        Some(STItem(path, board))
    }
}

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
        match write(&st_serde_filename, &st_json) {
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
        match read(&st_serde_filename) {
            Err(e) => {
                err(format!("Failed to read ST from disk: {:#?}", e));
                panic!("You should fix this");
            }
            Ok(json) => {
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

fn wait_to_simulate_lots_of_work() {
    let pause_duration = time::Duration::from_millis(900);
    thread::sleep(pause_duration);
}

// check if file or directory exists
pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

// add a breakpoint for testing
pub fn step() -> () {
    info_n("[press Enter to continue] ");
    let stdin = io::stdin();
    'step: for line in stdin.lock().lines() {
        match line.unwrap() {
            _ => break 'step,
        };
    }
}

pub fn solve(st: &mut ST) -> () {
    let max_depth: u16 = st.len().try_into().unwrap();
    // if max_depth is even then polarity is 0 else 1
    let polarity = match &max_depth {
        md if md % 2 == 0 => 0,
        _ => 1,
    };
    let new_node = vec![ST {
        v: Some((max_depth, polarity)),
        c: None,
    }];
    st.add_child(new_node);
    println!(": {:?}", max_depth);
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // import saved state if it exists
    let mut st: ST = fetch_st();
    let mut depth = st.len();

    // hook in the last_gasp function to capture
    //  and process any process Signals
    //  VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV
    //  VVVVVVVVVV This is the part we are thinking about VVVVVVVVVVVVV
    //  VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV
    // START last_gasp hook
    let signals = Signals::new(&[SIGINT, SIGHUP, SIGTERM, SIGQUIT])?;

    // lets stick a raw pointer to ST "in our back pocket" just in case the main
    // thread is sent a Signal (such as Ctrl+c or kill -HUP $pid)
    let st_ptr: *mut ST = &mut st; // this can not be safely passed to a thread

    let b = Wrapper(NonNull::new(st_ptr).unwrap()); // but this can!
    thread::spawn(move || {
        for sig in signals.forever() {
            let ptr = b.0.as_ptr();
            match sig {
                // 2 := Ctrl+c, 15 := -HUP
                // if we get a HUP then make a rapid backup of our data
                1 | 15 => unsafe { store_st(&(*ptr)) },
                // any other Signal causes use to attempt to make a backup and quit
                _ => unsafe { last_gasp(sig, &(*ptr)) },
            };
        }
    });
    //end of "evil" trap for signals; nothing to see here, lets get on with the actual work.
    //END last_gasp
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // ^^^^^^ This can't be the best way to do this ^^^^^^^^^^^^^^^^^
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    // do something work that takes a very long time
    // that we may need to interrupt and restart
    Ok(while depth < 1000 {
        print!(".");
        wait_to_simulate_lots_of_work();
        solve(&mut st);
        depth = st.len();
    })
}
