#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp::{min,max};
use std::collections::LinkedList;
use std::fmt::{self, Display};
use std::io::{self,Write};
use std::sync::{Arc,Condvar,Mutex};
use std::sync::mpsc::{channel, sync_channel, Sender};
use std::thread;
use std::time::Duration;

use semaphore::Semaphore;

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

fn condvar_demo(n: usize) {
    let mutex = Arc::new(Mutex::new(n));
    let cv    = Arc::new(Condvar::new());

    for i in 0 .. n {
        let mutex = mutex.clone();
        let cv    = cv.clone();

        thread::spawn(move || {
            let mut guard = mutex.lock().unwrap();

            while *guard != i {
                guard = cv.wait(guard).unwrap();
                println!("Thread {} wakes", i);
            }

            println!("Thread {} finishing", i);
        });
    }

    for i in 0 .. n {
        wait_for_enter();
        *mutex.lock().unwrap() = i;
        cv.notify_all();
    }
}

///////////////////////////////////////////////////////////////////////////////

const SEATS: usize = 3;
const HAIRCUT_MIN: u64 = 500;
const HAIRCUT_MAX: u64 = 1000;
const ARRIVAL_MIN: u64 = 250;
const ARRIVAL_MAX: u64 = 750;

fn sleeping_barber() {
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
                barber_ready.raise();
                customers_ready.lower();
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

fn sleeping_barber_2() {
    let seats: Arc<Mutex<LinkedList<(usize, Arc<Semaphore>)>>>
        = Arc::new(Mutex::new(LinkedList::new()));
    let customers_ready: Arc<Semaphore>
        = Arc::new(Semaphore::new(0));

    // Barber
    {
        let seats        = seats.clone();
        let customers_ready = customers_ready.clone();

        thread::spawn(move|| {
            loop {
                customers_ready.lower();
                let (i, ready) = seats.lock().unwrap().pop_front().unwrap();

                println!("Barber begins cutting customer {}", i);
                ready.raise();

                random_sleep(HAIRCUT_MIN, HAIRCUT_MAX);

                println!("Barber finishes cutting customer {}", i);
            }
        });
    }

    thread::spawn(move|| {
        for i in 0 .. {
            random_sleep(ARRIVAL_MIN, ARRIVAL_MAX);

            let seats = seats.clone();
            let customers_ready = customers_ready.clone();

            thread::spawn(move|| {
                let barber_ready: Arc<Semaphore>;

                {
                    let mut seats = seats.lock().unwrap();

                    if seats.len() == SEATS {
                        println!("Customer {} gives up", i);
                        return;
                    } else {
                        println!("Customer {} sits down", i);
                        barber_ready = Arc::new(Semaphore::new(0));
                        seats.push_back((i, barber_ready.clone()));
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

const NBARBERS: usize = 3;

fn sleeping_barbers() {
    type Message = (usize, Sender<usize>);
    let (queue_back, queue_front) = sync_channel::<Message>(SEATS);
    let queue_front = Arc::new(Mutex::new(queue_front));

    for j in 0 .. NBARBERS {
        let queue_front = queue_front.clone();

        thread::spawn(move || {
            let (i, response) = queue_front.lock().unwrap().recv().unwrap();
            response.send(j).unwrap();
            println!("Barber {} begins customer {}", j, i);
            random_sleep(HAIRCUT_MIN, HAIRCUT_MAX);
            println!("Barber {} finishes customer {}", j, i);
            response.send(j).unwrap();
        });
    }

    thread::spawn(move || {
        for i in 0 .. {
            random_sleep(ARRIVAL_MIN, ARRIVAL_MAX);

            let queue_back = queue_back.clone();

            thread::spawn(move || {
                let (response_send, response_recv) = channel();

                if let Ok(_) = queue_back.try_send((i, response_send)) {
                    println!("Customer {} sits down", i);
                    let j = response_recv.recv().unwrap();
                    println!("Customer {} being cut by barber {}", i, j);
                    response_recv.recv().unwrap();
                    println!("Customer {} done", i);
                } else {
                    println!("Customer {} gives up", i);
                }
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

    // condvar_demo(5);

    // sleeping_barber();
    // sleeping_barber_2();
    sleeping_barbers();

    wait_for_enter();
}
