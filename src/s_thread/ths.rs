use std::borrow::BorrowMut;
use std::cell::RefCell;
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

/*
=======
/* 
>>>>>>> 1c6bdd5 (update code)
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
<<<<<<< HEAD
 */
=======
*/

pub fn th_local() {
    thread_local! (static FOO:RefCell<u32> = RefCell::new(1));

    let FOO2:RefCell<String> = RefCell::new("hello".to_string());

    FOO.with(|f|{
        assert_eq!(*f.borrow(),1);
        *f.borrow_mut() = 2;
        println!("{:?}", f);
    });


    println!("FOO2 is {}", FOO2.take());

    let t = thread::spawn(move || {
        FOO.with(|f|{
            assert_eq!(*f.borrow(),1);
            *f.borrow_mut() = 3;
            println!("{:?}", f);
        });
    });

    t.join().unwrap();
    
    FOO.with(|f|{
        assert_eq!(*f.borrow(),2);
        println!("{:?}", f);
    });
}


use std::sync::Once;
pub fn th_only_once() {
    static mut VAL:usize = 0;
    static INIT: Once = Once::new();

    let th1 = thread::spawn(move ||{
        INIT.call_once(|| {
            unsafe {
                VAL = 1;
            }
        });
    });

    let th2 = thread::spawn(move ||{
        INIT.call_once(|| {
            unsafe {
                VAL = 2;
            }
        });
    });


    th1.join().unwrap();
    th2.join().unwrap();

    println!("{}", unsafe{VAL});

}
