use std::sync::{Mutex, Arc};

pub fn lock() {
    let m = Mutex::new(5);

    {
         let mut num = m.lock().unwrap();

         *num = 6;
    }



    println!("{:?}  ", m);
}


//Rc<T>/RefCell<T>用于单线程内部可变性， Arc<T>/Mutext<T>用于多线程内部可变性。
//Rc 多线程使用不安全

/*
use std::rc::Rc;
pub fn count_num() {
    let count = Rc::new(Mutex::new(0));
    let mut handlers = vec![];

    for i in 1..10 {
        let counter = Rc::clone(&count);
        // RC 不能保证并发的安全 所以下面编译会出现问题
        let handler = std::thread::spawn(move || {
            let mut num = count.lock().unwrap();

            *num += 1;
        });

        handlers.push(handler)
    }

    for handle in handlers{
        println!("start handle");
        handle.join().unwrap();
        println!("end handle");
    }

     println!("Result: {}", *count.lock().unwrap());
} 
*/

pub fn count_num_arc() {
    let count = Arc::new(Mutex::new(1));

    let mut handlers = vec![];

    for _ in 1..10{
        let counter = Arc::clone(&count);

        let hand = std::thread::spawn(move || {
            
           let mut num = counter.lock().unwrap();

           *num += 1;

        });

        handlers.push(hand);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("count num: {}", count.lock().unwrap());
}