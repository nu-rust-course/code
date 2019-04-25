#![allow(deprecated)]

use std::time::Duration;

use futures::Future;
use futures_cpupool::CpuPool;
use tokio_timer::Timer;

const BIG_PRIME: u64 = 15485867;
const TIMEOUT: u64 = 750;

fn main() {
    let pool = CpuPool::new_num_cpus();

    let prime = pool.spawn_fn(|| {
        Ok::<bool, ()>(is_prime(BIG_PRIME))
    });

    println!("Created the future");

//    if prime_future.wait().unwrap() {
//        println!("Prime");
//    } else {
//        println!("Not prime");
//    }

    let timeout = Timer::default()
        .sleep(Duration::from_millis(TIMEOUT))
        .then(|_| Err(()));

    let winner = timeout.select(prime).map(|(win, _)| win);

    match winner.wait() {
        Ok(true) => println!("Prime"),
        Ok(false) => println!("Not prime"),
        Err(_) => println!("Timed out"),
    }
}

/// Checks whether the given number is prime.
fn is_prime(num: u64) -> bool {
    for i in 2..num {
        if num % i == 0 { return false }
    }

    true
}

