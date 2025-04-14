use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {

    // java는 jvm이 추상화된 쓰레드를 만들어 사용하지만 러스트의 쓰레드는 진짜 os의 쓰레드를 사용한다

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawmed thread!", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    handle.join().unwrap();
    // 실행중인 쓰레드를 block 시킨다


    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(100));
    }

    thread_borrow();

    thread_tx_rx();

    thread_tx_rx_2();

    mutex();

    mutex_shared();
}

fn thread_borrow(){
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn thread_tx_rx(){
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("test");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn thread_tx_rx_2(){
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec! [
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
        let vals = vec! [
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}" ,received);
    }

}

fn mutex(){
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("mutex in here : {:?}", m);
    {
        let mut num2 = m.lock().unwrap();
        *num2 = 102;
    }
    println!("mutex in here : {:?}", m);

    println!("m = {:?}" , m);
}

fn mutex_shared(){
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

    println!("Result : {}", *counter.lock().unwrap());
}