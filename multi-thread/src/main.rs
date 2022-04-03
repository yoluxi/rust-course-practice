use std::thread;
use std::time::Duration;

fn main() {
    // thread_join
    // thread_move();
    // situation_3();
    // thread_barrier();
    // single_p_and_single_c();
    // transfer_ownership();
    for_recv();
}

fn thread_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //阻塞当前线程，等待子线程结束
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// move 所有权
fn thread_move() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

// 线程如何结束？
// 1. 线程代码执行完
// 2. 循环的I/O读取，
// 3. 循环没有设置设置终止条件，直到main线程结束
fn situation_3() {
    // 创建一个A线程
    let new_thread = thread::spawn(move || {
        // 创建一个B线程
        thread::spawn(move || loop {
            println!("I'am new thread");
        })
    });

    // 等待新创建的线程执行完
    new_thread.join().unwrap();
    println!("child thread is finished");

    // sleep 看B线程是否还在运行
    thread::sleep(Duration::from_millis(1000));
}

// 多线程性能
// 创建线程的消耗 ？
// 创建多少线程合适 ？CPU核心数
// 多线程的开销？ 锁，数据竞争，缓存失效

// 线程屏障
// 在线程间通过屏障来控制线程的执行顺序
use std::sync::{Arc, Barrier};
fn thread_barrier() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let c = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            c.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// 线程局部变量

// 用条件控制线程的挂起和执行  conditon_var 和 Mutex

// 只被调用一次的函数

// 线程中消息传递
use std::sync::mpsc;
fn single_p_and_single_c() {
    // 创建一个消息通道
    let (tx, rx) = mpsc::channel();
    // 创建一个线程,并发送消息
    thread::spawn(move || {
        // tx.send(1).unwrap();
        match tx.send(1) {
            Ok(_) => println!("send ok"),
            Err(e) => println!("send error: {}", e),
        }
    });
    // 在主线程中接受子线程发送的消息
    // rx.recv() 会阻塞当前线程
    // rx.try_recv() 不会阻塞当前线程
    println!("receive: {}", rx.recv().unwrap());
    // println!("receive: {}", rx.try_recv());
    // println!("receive: {}", rx.try_recv());
}

// 传输具有所有权的数据
// 1. 若值实现了COPY特征，直接复制
// 2. 若值没有实现COPY特征,  则他的所有权被转给接收端
fn transfer_ownership() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let s = String::from("hello");
        tx.send(s).unwrap();
        // error
        // println!("val is {}", s);
    });

    let received = rx.recv().unwrap();
    println!("got: {}", received);
}

// 使用for循环来接收消息
fn for_recv() {
    let (tx, rx) = mpsc::channel();
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

    for received in rx {
        println!("got: {}", received);
    }
}
