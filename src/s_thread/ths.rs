use std::thread;
use std::sync::Arc;
use std::time::Duration;
pub fn run_thread(){
    thread::spawn(move || {
       for i in 1..10 {
        println!("xxxxxxxx {} ", i);
        thread::sleep(Duration::from_millis(1));
       }
    });

    for i in 1..5{
        println!("yyyyyyyyyyyy {} ", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn run_thread_wait() {
    let handler = thread::spawn(|| {
        for i in 1..100 {
            println!("xxxxxxxx {} ", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handler.join().unwrap();

    for i in 1..5{
        println!("yyyyyyyyyyyy {} ", i);
        thread::sleep(Duration::from_millis(1));
    }

}

// 通过move 的关键字，转移所有权
pub fn run_thread_move() {
    let v = vec![1,2,3];

    let handler = std::thread::spawn(move || {
        println!("{:?}", v);
    });
    handler.join().unwrap();
}

pub fn run_thread_cas() {
let num_threads = 100;
for i in 0..num_threads {
    let ht = Arc::clone(&ht);

    let handle = thread::spawn(move || {
        for j in 0..adds_per_thread {
            let key = thread_rng().gen::<u32>();
            let value = thread_rng().gen::<u32>();
            ht.set_item(key, value);
        }
    });

    handles.push(handle);
}

}