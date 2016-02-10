#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp::{min,max};
use std::fmt::{self, Display};
use std::io::{self,Write};
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

extern crate rand;

mod semaphore;

///////////////////////////////////////////////////////////////////////////////

fn wait_for_enter() {
    println!("Press enter to continue");
    io::stdout().flush().expect("flush");

    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).expect("read_line");
}

///////////////////////////////////////////////////////////////////////////////

const COUNT_TO_DELAY: u64 = 500;

// Counts from 1 to n, printing each
fn count_to(who: usize, n: usize)
{
    for i in 1 .. (n + 1) {
        thread::sleep(Duration::from_millis(COUNT_TO_DELAY));
        println!("{} says {}", who, i);
    }
}

// Starts two counters concurrently
fn two_counters() {
    let n = 5;
    let handle1 = thread::spawn(move || count_to(1, n));
    let handle2 = thread::spawn(move || count_to(2, n));

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn n_counters(n: usize) {
    let mut handles = vec![];

    for i in 0 .. n {
        handles.push(thread::spawn(move || count_to(i, 5)));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

///////////////////////////////////////////////////////////////////////////////

// Sleeps for for between min and max milliseconds
fn random_sleep(min: u64, max: u64) {
    let millis = if min == max {min}
                 else {min + (rand::random::<u64>() % (max - min))};
    thread::sleep(Duration::from_millis(millis));
}

///////////////////////////////////////////////////////////////////////////////

struct Chopstick {
    which: String,
    uses:  usize,
}

impl Chopstick {
    fn new<N: Display>(which: N) -> Self {
        Chopstick {
            which: format!("{}", which),
            uses:  0
        }
    }

    fn eat(&mut self) {
        self.uses += 1;
    }
}

impl Display for Chopstick {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt.write_fmt(format_args!("chopstick {} (use {})",
                                   self.which, self.uses))
    }
}

const DP_CSLATENCY_MIN: u64 = 125;
const DP_CSLATENCY_MAX: u64 = 250;
const DP_SLEEP_MIN: u64 = 250;
const DP_SLEEP_MAX: u64 = 500;
const DP_EAT_MIN: u64 = 2000;
const DP_EAT_MAX: u64 = 3000;

fn sleep<N: Display>(who: N) {
    println!("Philosopher {} goes to sleep", who);
    random_sleep(DP_SLEEP_MIN, DP_SLEEP_MAX);
}

fn eat<N: Display>(who: N, cs1: &mut Chopstick, cs2: &mut Chopstick) {
    cs1.eat(); cs2.eat();
    println!("Philosopher {} eats with {} and {}", who, cs1, cs2);
    random_sleep(DP_SLEEP_MIN, DP_SLEEP_MAX);
}

fn two_dining_philosophers() {
    let cs_a = Arc::new(Mutex::new(Chopstick::new("A")));
    let cs_b = Arc::new(Mutex::new(Chopstick::new("B")));

    let cs_a2 = cs_a.clone();
    let cs_b2 = cs_b.clone();

    let philosopher1 = thread::spawn(move|| {
        loop {
            {
                let mut guard_a = cs_a.lock().unwrap();
                random_sleep(DP_CSLATENCY_MIN, DP_CSLATENCY_MAX);
                let mut guard_b = cs_b.lock().unwrap();
                eat(1, &mut guard_a, &mut guard_b);
            }

            sleep(1);
        }
    });

    let philosopher2 = thread::spawn(move|| {
        loop {
            {
                let mut guard_a = cs_a2.lock().unwrap();
                random_sleep(DP_CSLATENCY_MIN, DP_CSLATENCY_MAX);
                let mut guard_b = cs_b2.lock().unwrap();
                eat(2, &mut guard_a, &mut guard_b);
            }

            sleep(2);
        }
    });
}

///////////////////////////////////////////////////////////////////////////////

fn dining_philosophers(n: usize) {
    let chopsticks: Vec<Arc<Mutex<Chopstick>>> =
        (0 .. n).map(|i| Arc::new(Mutex::new(Chopstick::new(i)))).collect();

    for i in 0 .. n {
        let j = (i + 1) % n;

        let cs_i = chopsticks[min(i, j)].clone();
        let cs_j = chopsticks[max(i, j)].clone();

        thread::spawn(move|| {
            loop {
                {
                    let mut guard_i = cs_i.lock().unwrap();
                    println!("Philosopher {} picks up {}", i, *guard_i);
                    random_sleep(DP_CSLATENCY_MIN, DP_CSLATENCY_MAX);
                    let mut guard_j = cs_j.lock().unwrap();
                    println!("Philosopher {} picks up {}", i, *guard_j);

                    eat(i, &mut guard_i, &mut guard_j);
                }

                sleep(i);
            }
        });

    }
}

///////////////////////////////////////////////////////////////////////////////

const SEATS: usize = 3;
const HAIRCUT_MIN: u64 = 1000;
const HAIRCUT_MAX: u64 = 1000;
const ARRIVAL_MIN: u64 = 250;
const ARRIVAL_MAX: u64 = 750;

fn sleeping_barber() {
    use semaphore::Semaphore;

    let free_seats      = Arc::new(Mutex::new(SEATS));
    let customers_ready = Arc::new(Semaphore::new(0));
    let barber_ready    = Arc::new(Semaphore::new(0));

    // Barber
    {
        let free_seats = free_seats.clone();
        let customers_ready = customers_ready.clone();
        let barber_ready = barber_ready.clone();

        thread::spawn(move|| {
            loop {
                customers_ready.lower();
                barber_ready.raise();
                *free_seats.lock().unwrap() += 1;

                println!("Barber begins cutting");
                random_sleep(HAIRCUT_MIN, HAIRCUT_MAX);
                println!("Barber finishes cutting");
            }
        });
    }

    thread::spawn(move|| {
        for i in 0 .. {
            random_sleep(ARRIVAL_MIN, ARRIVAL_MAX);

            let free_seats = free_seats.clone();
            let customers_ready = customers_ready.clone();
            let barber_ready = barber_ready.clone();

            thread::spawn(move|| {
                {
                    let mut free_seats_guard = free_seats.lock().unwrap();

                    if *free_seats_guard == 0 {
                        println!("Customer {} gives up", i);
                        return;
                    } else {
                        println!("Customer {} sits down", i);
                        *free_seats_guard -= 1;
                        customers_ready.raise();
                    }
                }

                barber_ready.lower();
                println!("Customer {} begins getting cut", i);
            });
        }
    });
}

///////////////////////////////////////////////////////////////////////////////

fn main() {
    // n_counters(5);
    // refcounting();
    // two_dining_philosophers();

    // dining_philosophers(5);

    sleeping_barber();

    wait_for_enter();
}
