use crate::log::info_n;
use crate::our_data::st_or_to_disk::fetch_st;
use crate::our_data::ST;
use std::error::Error;
use std::fs;
use std::{io, io::prelude::*}; // fn step()
use std::{thread, time}; //fs::metadata for path_exists

fn wait_to_simulate_lots_of_work() {
    let pause_duration = time::Duration::from_millis(900);
    thread::sleep(pause_duration);
}

mod fsio;
mod last_gasp;
mod log;
mod our_data;
mod work;

pub fn run() -> Result<(), Box<dyn Error>> {
    // import saved state if it exists
    let mut st: ST = fetch_st();
    let mut depth = st.len();

    // hook in the last_gasp function to capture
    //  and process any process Signals
    //  VVVVVVVVVVVVVVVVVVVVVV This is the part we are thinking about
    let _ = crate::last_gasp::hook(&mut st);

    // do something work that takes a very long time
    // that we may need to interrupt and restart
    Ok(while depth < 1000 {
        print!(".");
        wait_to_simulate_lots_of_work();
        crate::work::solve(&mut st);
        depth = st.len();
    })
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
