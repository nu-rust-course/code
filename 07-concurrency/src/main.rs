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

use semaphore::Semaphore;

mod semaphore;

///////////////////////////////////////////////////////////////////////////////

/// Waits for the user to press Enter.
fn wait_for_enter() {
    println!("Press enter to continue");
    io::stdout().flush().expect("flush");
    io::stdin().read_line(&mut String::new()).expect("read_line");
}

///////////////////////////////////////////////////////////////////////////////

const COUNT_TO_DELAY: u64 = 500;

/// Counts from 1 to n, printing each
fn count_to(who: usize, n: usize)
{
    for i in 1 ..= n {
        thread::sleep(Duration::from_millis(COUNT_TO_DELAY));
        println!("{} says {}", who, i);
    }
}

/// Starts two counters concurrently, then waits for each to finish.
fn two_counters() {
    let n = 5;
    let handle1 = thread::spawn(move || count_to(1, n));
    let handle2 = thread::spawn(move || count_to(2, n));

    handle1.join().unwrap();
    handle2.join().unwrap();
}

/// Starts `n` counters concurrently, then waits for each to finish.
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

const MUTEX_DEMO_SLEEP: u64 = 100;

fn mutex_demo(nthreads: usize) {
    let counter = Arc::new(Mutex::new(0));

    for i in 0 .. nthreads {
        let counter = counter.clone();

        thread::spawn(move || {
            thread::sleep(Duration::from_millis(MUTEX_DEMO_SLEEP));

            let mut guard = counter.lock().unwrap();
            println!("Thread {} sees counter = {}", i, *guard);
            *guard += 1;
        });
    }
}

///////////////////////////////////////////////////////////////////////////////

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

    fn eat_with(&mut self, other: &mut Chopstick) {
        self.uses += 1;
        other.uses += 1;
    }
}

impl Display for Chopstick {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "chopstick {} ({} uses)", self.which, self.uses)
    }
}

const DP_CSLATENCY_MIN: u64 = 125;
const DP_CSLATENCY_MAX: u64 = 250;
const DP_SLEEP_MIN: u64 = 250;
const DP_SLEEP_MAX: u64 = 1000;
const DP_EAT_MIN: u64 = 250;
const DP_EAT_MAX: u64 = 500;

fn sleep<N: Display>(who: N) {
    println!("Philosopher {} goes to sleep", who);
    random_sleep(DP_SLEEP_MIN, DP_SLEEP_MAX);
    println!("Philosopher {} wakes up", who);
}

fn pick_up<N: Display>(who: N, cs: &Mutex<Chopstick>) -> MutexGuard<Chopstick> {
    let guard = cs.lock().unwrap();
    println!("Philosopher {} picks up {}", who, *guard);
    random_sleep(DP_CSLATENCY_MIN, DP_CSLATENCY_MAX);
    guard
}

fn eat<N: Display>(who: N, cs1: &mut Chopstick, cs2: &mut Chopstick) {
    cs1.eat_with(cs2);
    println!("Philosopher {} eats with {} and {}", who, cs1, cs2);
    random_sleep(DP_EAT_MIN, DP_EAT_MAX);
}

fn two_dining_philosophers() {
    let cs_a1 = Arc::new(Mutex::new(Chopstick::new("A")));
    let cs_b1 = Arc::new(Mutex::new(Chopstick::new("B")));

    let cs_a2 = cs_a1.clone();
    let cs_b2 = cs_b1.clone();

    let philosopher1 = thread::spawn(move|| {
        loop {
            {
                let mut guard_a = pick_up(1, &cs_a1);
                let mut guard_b = pick_up(1, &cs_b1);
                eat(1, &mut guard_a, &mut guard_b);
            }

            sleep(1);
        }
    });

    let philosopher2 = thread::spawn(move|| {
        loop {
            {
                let mut guard_a = pick_up(2, &cs_a2);
                let mut guard_b = pick_up(2, &cs_b2);
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

        let cs1 = chopsticks[min(i, j)].clone();
        let cs2 = chopsticks[max(i, j)].clone();

        thread::spawn(move|| {
            loop {
                {
                    let mut guard1 = pick_up(i, &cs1);
                    let mut guard2 = pick_up(i, &cs2);
                    eat(i, &mut guard1, &mut guard2);
                }

                sleep(i);
            }
        });
    }
}

///////////////////////////////////////////////////////////////////////////////

fn dining_philosophers_one_arc(n: usize) {
    let chopsticks: Arc<Vec<Mutex<Chopstick>>> =
        Arc::new((0 .. n).map(|i| Mutex::new(Chopstick::new(i))).collect());

    for i in 0 .. n {
        let chopsticks = chopsticks.clone();

        thread::spawn(move|| {
            loop {
                let j   = (i + 1) % n;
                let cs1 = &chopsticks[min(i, j)];
                let cs2 = &chopsticks[max(i, j)];

                {
                    let mut guard1 = pick_up(i, &cs1);
                    let mut guard2 = pick_up(i, &cs2);
                    eat(i, &mut guard1, &mut guard2);
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
const ARRIVAL_MIN: u64 = 150;
const ARRIVAL_MAX: u64 = 250;

#[derive(Debug)]
struct BarberState {
    free_seats:      Mutex<usize>,
    customers_ready: Semaphore,
    barber_ready:    Semaphore,
}

fn sleeping_barber() {
    let state = Arc::new(BarberState {
        free_seats:      Mutex::new(SEATS),
        customers_ready: Semaphore::new(0),
        barber_ready:    Semaphore::new(0),
    });

    // Barber
    {
        let state = state.clone();

        thread::spawn(move|| {
            loop {
                state.barber_ready.raise();
                state.customers_ready.lower();
                *state.free_seats.lock().unwrap() += 1;

                println!("Barber begins cutting");
                random_sleep(HAIRCUT_MIN, HAIRCUT_MAX);
                println!("Barber finishes cutting");
            }
        });
    }

    // Customers
    thread::spawn(move|| {
        for i in 0 .. {
            random_sleep(ARRIVAL_MIN, ARRIVAL_MAX);

            let state = state.clone();

            thread::spawn(move|| {
                {
                    let mut free_seats_guard = state.free_seats.lock().unwrap();

                    if *free_seats_guard == 0 {
                        println!("Customer {} gives up", i);
                        return;
                    } else {
                        println!("Customer {} sits down", i);
                        *free_seats_guard -= 1;
                        state.customers_ready.raise();
                    }
                }

                state.barber_ready.lower();
                println!("Customer {} begins getting cut", i);
            });
        }
    });
}

fn sleeping_barber_2() {
    type CustomerList = LinkedList<(usize, Arc<Semaphore>)>;

    let seats: Arc<Mutex<CustomerList>>
        = Arc::new(Mutex::new(CustomerList::new()));
    let customers_ready: Arc<Semaphore>
        = Arc::new(Semaphore::new(0));

    // Barber
    {
        let seats           = seats.clone();
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
    // protocol: C:usize; B:usize; B:()
    type Message = (usize, SyncSender<usize>);
    let (queue_back, queue_front) = sync_channel::<Message>(SEATS);
    let queue_front = Arc::new(Mutex::new(queue_front));

    for j in 0 .. NBARBERS {
        let queue_front = queue_front.clone();

        thread::spawn(move || {
            loop {
                let (i, response) = queue_front.lock().unwrap().recv().unwrap();

                println!("Barber {} begins customer {}", j, i);
                response.send(j).unwrap();
                random_sleep(HAIRCUT_MIN, HAIRCUT_MAX);
                response.send(j).unwrap();
                println!("Barber {} finishes customer {}", j, i);
            }
        });
    }

    thread::spawn(move || {
        for i in 0 .. {
            random_sleep(ARRIVAL_MIN, ARRIVAL_MAX);

            let queue_back = queue_back.clone();

            thread::spawn(move || {
                let (response_send, response_recv) = sync_channel(0);

                if queue_back.try_send((i, response_send)).is_ok() {
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
