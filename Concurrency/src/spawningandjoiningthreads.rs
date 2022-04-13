use std::thread;
use std::time;

pub fn demo(){
    let handle = thread::spawn(|| {
        for _ in 1..10 {
            print!("+");
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    for _ in 1..10 {
        print!("-");
        thread::sleep(time::Duration::from_millis(300));
    }

    // Wait for the thread spawned to finish
    // If not --> main thread execution could finish previous to finish the execution to the another thread
    handle.join();
}