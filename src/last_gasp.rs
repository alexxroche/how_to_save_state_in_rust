extern crate signal_hook;
use crate::our_data::st_or_to_disk::store_st;
use crate::ST;
use signal_hook::{iterator::Signals, SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use std::error::Error;
use std::thread;

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

pub static SV: AtomicI32 = AtomicI32::new(0);
pub static TRAFFIC_LIGHT: AtomicBool = AtomicBool::new(false);

/// clear_sig after signal has been processed
pub fn clear_sig() -> () {
    // no need to clear the actual SV
    set_tl(false)
}

fn set_tl(v: bool) {
    TRAFFIC_LIGHT.store(v, Ordering::Relaxed);
    //make_fds_true();
}
/// is_sig shows if there is a signal waiting to be processed
pub fn is_sig() -> bool {
    TRAFFIC_LIGHT.load(Ordering::Relaxed)
}
/// get_sig gets the most recent signal
pub fn get_sig() -> i32 {
    SV.load(Ordering::Relaxed)
}

/*
thread_local! {
    pub static CURRENT_SIGNAL: RwLock<Arc<SignalValue>> = RwLock::new(Default::default());
    pub static FDS: RwLock<bool> = RwLock::new(false);
}
*/

/// This is triggered by the Signals::
pub fn last_gasp(sig: i32, st: &ST) -> () {
    // this should take a function as an argument so that
    // this can be an external crate
    println!("Writing current ST to disk");
    store_st(&st);
    // write board_to_html while we are at it
    println!("Files written (phew!); exiting now {:#?}", sig);
    std::process::exit(sig);
}

/// hook to trap signals
pub fn hook() -> Result<(), Box<dyn Error>> {
    let signals = Signals::new(&[SIGINT, SIGHUP, SIGTERM, SIGQUIT])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            // set the signal
            SV.store(sig, Ordering::SeqCst);
            // flag the system to read the new signal
            set_tl(true);
        }
    });
    Ok(())
    //END last_gasp
}
