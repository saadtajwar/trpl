use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::{Mutex, Arc};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap())


    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {m:?}")

    // let (tx, rx) = mpsc::channel();
    // let tx1 = tx.clone();

    // thread::spawn(move || {
    //     // let val = String::from("hi");
    //     let vals = vec![
    //         String::from("m1"),
    //         String::from("m2"),
    //         String::from("m3"),
    //         String::from("m4"),
    //     ];
        
    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    //     // println!("val is {val}");
    // });

    // thread::spawn(move || {
    //     // let val = String::from("hi");
    //     let vals = vec![
    //         String::from("c1"),
    //         String::from("c2"),
    //         String::from("c3"),
    //         String::from("c4"),
    //     ];
        
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    //     // println!("val is {val}");
    // });


    // for received in rx {
    //     println!("Got {received}");
    // }

    // println!("After")

    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {v:?}");
    // });

    // handle.join().unwrap();
    // println!("Here's a vector: {v:?}");
}