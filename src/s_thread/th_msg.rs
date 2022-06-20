use std::sync::mpsc;
use std::thread;
use std::time::Duration;
pub fn ch_mpsc() {
   let (tx , rx )  = mpsc::channel();

   thread::spawn(move || {
        tx.send(1).unwrap();
        tx.send(1).unwrap();
   });

   // 如果发送者退出。 则recv 会接受到一个错误
   println!("receive {}", rx.recv().unwrap());
   
   // 不阻塞读取
   //println!("receive {}", rx.try_recv().unwrap());
   //println!("receive {}", rx.try_recv().unwrap());
}

pub fn send_recv() {
    let(tx,rx) = mpsc::channel();

    //多发送者需要 clone tx 
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    for rec in rx {
        println!("rec {}", rec);
    }

}