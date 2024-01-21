use std::{thread, time::Duration};

#[derive(Clone, Copy, Debug)]
struct Worker(usize);

const WORKER_COUNT: usize = 8;

#[derive(Clone, Copy, Debug)]
struct Arbitrator([Worker;WORKER_COUNT]);

static mut ARBITRATOR: Arbitrator = Arbitrator([Worker(0); WORKER_COUNT]);

fn arbitrator() -> &'static mut Arbitrator {
    unsafe { &mut ARBITRATOR }
}

fn run_thread(parent: &'static mut Arbitrator, x: usize) {
    let worker = unsafe { parent.0.get_unchecked_mut(x) };
    *worker = Worker(x);
    thread::spawn(move || {
        println!("[{:?}] spawning...", worker);
        thread::sleep(Duration::from_millis(1000));
    });
}

fn spawn_workers() {
    let mut i = 0;
    while i < WORKER_COUNT {
        run_thread(arbitrator(), i);
        i += 1;
    }
}

pub fn run() {
    thread::spawn(|| {
        println!("[{:?}] initializing...", arbitrator());
        thread::sleep(Duration::from_millis(1000));
    });

    spawn_workers();
}