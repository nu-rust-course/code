#![allow(unused)]
#![allow(clippy::mutex_atomic)]

use std::{
    cmp::{min, max},
    collections::LinkedList,
    fmt::{self, Display},
    io::{self, Write},
    sync::{Arc, Condvar, Mutex, MutexGuard, mpsc::{sync_channel, SyncSender}},
    thread,
    time::Duration,
};

use rand::{Rng, thread_rng, distributions::Uniform};

//use semaphore::Semaphore;

mod semaphore;
mod logging;

const COUNT_TO_DELAY: u64 = 500;

/// Counts from 1 to n, printing each
fn count_to(who: usize, n: usize)
{
    for i in 1 ..= n {
        thread::sleep(Duration::from_millis(COUNT_TO_DELAY));
        println!("{} says {}", who, i);
    }
}

///////////////////////////////////////////////////////////////////////////////

/*
 * Useful little helpers:
 */

/// Prints a message and waits for the user to press enter.
fn wait_for_enter() {
    println!("Press enter to continue");
    io::stdout().flush().expect("flush");
    io::stdin().read_line(&mut String::new()).expect("read_line");
}

/// Sleeps for between min and max milliseconds
///
/// # Panics
///
/// Panics if `min > max`.
fn random_sleep(min: u64, mut max: u64) {
    let dist = Uniform::new(Duration::from_millis(min),
                            Duration::from_millis(max + 1));
    let rng = &mut thread_rng();
    thread::sleep(rng.sample(dist));
}

///////////////////////////////////////////////////////////////////////////////

fn main() {
    // two_counters();
    // n_counters(5);

    // mutex_demo(10);

    // two_dining_philosophers();
    // dining_philosophers(5);
    // dining_philosophers_one_arc(5);

    // condvar_demo(5);

    // sleeping_barber();
    // sleeping_barber_2();
    // sleeping_barbers();

    wait_for_enter();
}

