use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let count = Arc::new(Mutex::new(0));
    let mut handlers = Vec::new();

    for i in 1..=10 {
        // creating clone of count, so ownership won't move to thread spawned closure
        // rust's wizzles ¯\_(ツ)_/¯
        let count_clone = Arc::clone(&count);

        let handle = thread::spawn(move || {
            // generate random delay inside thread
            let mut gen = thread_rng();
            let random_number = gen.gen_range(1..=10);
            thread::sleep(Duration::from_secs(random_number));

            // fun part
            // lock mutex so we would have access to it
            let mut state = count_clone.lock().unwrap();

            // dereferencing needed, because state is MutexGuard and looks like this:
            // state is Mutex { data: 10, poisoned: false, .. }
            // this is how to get an access to data directly
            *state += 1;

            // just log
            println!("Tread number {} changed count state {:?}", i, state);
        });

        // we need to join all threads later, otherwise main will end earlier
        // than all threads woke up and done with their job
        handlers.push(handle);
    }

    // wait all
    for handle in handlers {
        handle.join().unwrap();
    }

    // once again, count is MutexGuard
    // so get access, then dereferencing to get it's value (data)
    let final_state = count.lock().unwrap();
    println!("Count is {:?}", *final_state);
}
