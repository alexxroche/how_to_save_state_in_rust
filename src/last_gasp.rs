// listen for the kill signals
extern crate signal_hook;
use crate::our_data::st_or_to_disk::store_st;
use crate::ST;
use signal_hook::{iterator::Signals, SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use std::error::Error;
use std::ptr::NonNull;
use std::thread;

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

pub fn hook(st: &mut ST) -> Result<(), Box<dyn Error>> {
    // START last_gasp hook
    let signals = Signals::new(&[SIGINT, SIGHUP, SIGTERM, SIGQUIT])?;

    // lets stick a raw pointer to ST "in our back pocket" just in case the main
    // thread is sent a Signal (such as Ctrl+c or kill -HUP $pid)
    let st_ptr: *mut ST = st; // this can not be safely passed to a thread

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
    Ok(())
    //end of "evil" trap for signals; nothing to see here, lets get on with the actual work.
    //END last_gasp
}
