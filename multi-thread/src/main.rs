use std::rc::Rc;
use std::thread;
use std::time::Duration;

fn main() {
    // thread_join
    // thread_move();
    // situation_3();
    // thread_barrier();
    // single_p_and_single_c();
    // transfer_ownership();
    // for_recv();
    // mpmc();
    // async_channel();
    // sync_channel();
    // single_thread_mutex();
    // multiple_thread_mutex();
    // single_thread_dead_lock();
    // multiple_thread_dead_lock();
    // multiple_thread_try_lock();
    // rw_lock();
    // condvar_test();
    semaphore_test();
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
use std::sync::{Arc, Barrier, Condvar, Mutex, MutexGuard, RwLock};
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

// mpmc
fn mpmc() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send("1").unwrap();
    });

    thread::spawn(move || {
        tx1.send("2").unwrap();
    });

    for received in rx {
        println!("got: {}", received);
    }
}

// 同步和异步通道
fn async_channel() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        // let val = String::from("hi");
        println!("send before");
        tx.send("1").unwrap();
        println!("send after");
    });

    println!("sleep before");
    thread::sleep(Duration::from_secs(3));
    println!("sleep after");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

// 同步通道
fn sync_channel() {
    // sync_channel(n) n代表消息缓存的条数
    let (tx, rx) = mpsc::sync_channel(1);
    let handle = thread::spawn(move || {
        // let val = String::from("hi");
        println!("send before");
        tx.send("1").unwrap();
        println!("send after");
    });

    println!("sleep before");
    thread::sleep(Duration::from_secs(3));
    println!("sleep after");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

// 互斥锁 mutex(mutual exclusion)
// 让多个线程并发访问同一个值变成了排队访问
fn single_thread_mutex() {
    // 创建互斥锁的实例
    use std::sync::Mutex;
    let m = Mutex::new(5);
    {
        // 获取锁，然后deref为m的引用
        // lock返回Result
        let mut num = m.lock().unwrap();
        *num = 6;
        // 锁自动被drop
    }
    println!("m is {:?}", m);
}

fn multiple_thread_mutex() {
    // 使用Arc实现mutex的多所有权
    // Rc<T>/RefCell<T>用于单线程内部可变性， Arc<T>/Mutext<T>用于多线程内部可变性。
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
    // 等待所有子线程执行完毕
    for handle in handles {
        handle.join().unwrap();
    }
    // 输出最后的统计结果
    println!("Result: {}", *counter.lock().unwrap());
}

// 死锁
// 同时拿两个资源，一个锁还没释放，就立即用另一个锁住资源，这就是死锁
fn single_thread_dead_lock() {
    let data = Mutex::new(0);
    let d1 = data.lock();
    let d2 = data.lock();
}

// 多线程死锁
use lazy_static::lazy_static;
lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}
fn multiple_thread_dead_lock() {
    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            if i_thread == 0 {
                // 锁住MUTEX1
                let guard = MUTEX1.lock().unwrap();
                println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);
                // 当前线程睡眠一会，等待线程2锁住MUTEX2
                thread::sleep(Duration::from_secs(1));
                // 锁住MUTEX2
                let guard = MUTEX2.lock().unwrap();
            } else {
                let _guard = MUTEX2.lock().unwrap();
                println!("线程 {} 锁住了MUTEX2，接着准备去锁MUTEX1 !", i_thread);
                let _guard = MUTEX1.lock().unwrap();
            }
        }));
    }

    // 等待所有子线程执行完毕
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生！");
}

fn multiple_thread_try_lock() {
    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                // 线程1
                if i_thread % 2 == 0 {
                    // 锁住MUTEX1
                    let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                    // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                    thread::sleep(Duration::from_millis(10));

                    // 去锁MUTEX2
                    let guard = MUTEX2.try_lock();
                    println!("线程1获取MUTEX2锁的结果: {:?}", guard);
                // 线程2
                } else {
                    // 锁住MUTEX2
                    let _guard = MUTEX2.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);
                    thread::sleep(Duration::from_millis(10));
                    let guard = MUTEX1.try_lock();
                    println!("线程2获取MUTEX1锁的结果: {:?}", guard);
                }
            }
        }));
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");
}

// 读写锁 RwLock
fn rw_lock() {
    let lock = RwLock::new(5);
    // 同一时间允许多个读
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    // 只允许一个写
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);

        // 读和写不能同时存在
        let _r = lock.read().unwrap();
        println!("{:?}", _r);
    }
}

// 追求高并发读取时，使用RwLock，因为Mutex一次只允许一个线程去读取
// 如果要保证写操作的成功性，使用Mutex
// 不知道哪个合适，统一使用Mutex

// 用条件变量condvar实现线程同步
// mutex 资源安全访问
// condvar 访问顺序
fn condvar_test() {
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let handle = thread::spawn(move || {
        let mut m = { *cflag.lock().unwrap() };
        let mut counter = 0;

        while counter < 3 {
            while !m {
                m = *ccond.wait(cflag.lock().unwrap()).unwrap();
            }

            {
                m = false;
                *cflag.lock().unwrap() = false;
            }

            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        thread::sleep(Duration::from_secs(1));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outer counter: {}", counter);
        cond.notify_one();
    }

    handle.join().unwrap();
    println!("{:?}", flag);
}

// 信号量
// 精准的控住当前正在运行的任务最大数量

use tokio::sync::Semaphore;
#[tokio::main]
async fn semaphore_test() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut handles = Vec::new();

    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        handles.push(tokio::spawn(async move {
            println!("{:?}", permit);
            drop(permit);
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

// Auomic 原子类型
// 内部使用了CAS循环
// CAS 全称是 Compare and swap, 它通过一条指令读取指定的内存地址，然后判断其中的值是否等于给定的前置值，如果相等，则将其修改为新的值

// Send + Sync
// 实现Send的类型可以在线程间安全的传递其所有权
// 实现Sync的类型可以在线程间安全的共享(通过引用)
