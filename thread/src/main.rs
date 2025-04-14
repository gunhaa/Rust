use std::thread;
use std::time::Duration;

fn main() {

    // java는 jvm이 추상화된 쓰레드를 만들어 사용하지만 러스트의 쓰레드는 진짜 os의 쓰레드를 사용한다
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawmed thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

}
