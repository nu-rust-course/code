#![allow(dead_code)]
#![allow(unused_variables)]

extern crate rand;

use std::fmt::{self, Display};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::thread;

//////////////////////////////////////////////////////////////////////

const COUNT_TO_DELAY: u64 = 500;
const COUNT_TO_MAX: usize = 5;

fn count_to(n: usize) {
    for i in 1 .. (n + 1) {
        thread::sleep(Duration::from_millis(COUNT_TO_DELAY));
        println!("{}", i);
    }
}

fn do_counting() {
    let t1 = thread::spawn(move || count_to(COUNT_TO_MAX));
    let t2 = thread::spawn(move || count_to(COUNT_TO_MAX));
    t1.join().unwrap();
    t2.join().unwrap();
}

fn do_more_counting(n: usize) {
    let mut v = vec![];

    for _ in 0 .. n {
        v.push(thread::spawn(move || count_to(COUNT_TO_MAX)));
    }

    for handle in v {
        handle.join().unwrap();
    }
}

//////////////////////////////////////////////////////////////////////

fn random_sleep(min: u64, max: u64) {
    let millis = min + (rand::random::<u64>() % (max - min));
    thread::sleep(Duration::from_millis(millis));
}

//////////////////////////////////////////////////////////////////////

struct Fork {
    which: String,
    uses:  usize,
}

impl Fork {
    fn new<N: Display>(which: N) -> Self {
        Fork {
            which: format!("{}", which),
            uses:  0,
        }
    }

    fn eat(&mut self) {
        self.uses += 1;
    }
}

impl Display for Fork {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("fork {} (use {})",
                                   self.which, self.uses))
    }
}

const DP_SLEEP_MIN: u64 = 250;
const DP_SLEEP_MAX: u64 = 500;
const DP_EAT_MIN: u64   = 250;
const DP_EAT_MAX: u64   = 500;

fn sleep<N: Display>(who: N) {
    println!("Philosopher {} goes to sleep", who);
    random_sleep(DP_SLEEP_MIN, DP_SLEEP_MAX);
}

fn eat<N: Display>(who: N, fork1: &mut Fork, fork2: &mut Fork) {
    fork1.eat(); fork2.eat();
    println!("Philosopher {} eats with {} and {}", who, fork1, fork2);
    random_sleep(DP_EAT_MIN, DP_EAT_MAX);
}

fn two_dining_philosophers() {
    let fork_a1 = Arc::new(Mutex::new(Fork::new("A")));
    let fork_b1 = Arc::new(Mutex::new(Fork::new("B")));

    let fork_a2 = fork_a1.clone();
    let fork_b2 = fork_b1.clone();

    let philosopher1 = thread::spawn(move || {
        loop {
            {
                let mut guard_a = fork_a1.lock().unwrap();
                let mut guard_b = fork_b1.lock().unwrap();
                eat("1", &mut guard_a, &mut guard_b);
            }

            sleep("1");
        }
    });

    let philosopher2 = thread::spawn(move || {
        loop {
            {
                let mut guard_a = fork_a2.lock().unwrap();
                let mut guard_b = fork_b2.lock().unwrap();
                eat("2", &mut guard_a, &mut guard_b);
            }

            sleep("2");
        }
    });

    philosopher1.join().unwrap();
    philosopher2.join().unwrap();
}

fn build_vec<T, F>(size: usize, mut init: F) -> Vec<T>
    where F: FnMut(usize) -> T
{
    let mut v = Vec::with_capacity(size);
    for i in 0 .. size {
        v.push(init(i));
    }
    v
}

fn dining_philosophers(n: usize) {
    let forks = build_vec(n, |i| Mutex::new(Fork::new(i)));
    let arc   = Arc::new(forks);

    for i in 0 .. n {
        let arc = arc.clone();



    }
}

//////////////////////////////////////////////////////////////////////

fn main() {
    two_dining_philosophers();
}
