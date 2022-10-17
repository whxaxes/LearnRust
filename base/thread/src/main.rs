use std::{thread, time::Duration, sync::{Arc, Barrier, mpsc, Mutex, RwLock}};
use thread_local::ThreadLocal;
use std::cell::Cell;
use global_util::TestUtil;
use flume;

fn main() {
    TestUtil::wrap_with_label("normal", || {
        let val = "111";

        let handler = thread::spawn(move || {
            for i in 0..10 {
                println!("in thread {:?}, {}", i, val);
                thread::sleep(Duration::from_millis(1));
            }
        });

        // 阻塞等上面线程执行完
        handler.join().unwrap();

        for i in 0..5 {
            println!("not in thread {:?}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    TestUtil::wrap_with_label("barrier", || {
        // 线程屏障
        let bar = Arc::new(Barrier::new(6));
        let mut handlers = Vec::with_capacity(6);
        for i in 0..6 {
            let b = bar.clone();
            handlers.push(thread::spawn(move || {
                println!("before {:?}", i);
                b.wait();
                println!("after {:?}", i);
            }));
        }

        for h in handlers {
            h.join().unwrap();
        }
    });

    TestUtil::wrap_with_label("thread_local", || {
        // 多线程变量累加
        let tls = Arc::new(ThreadLocal::new());
        for _ in 0..6 {
            let t = tls.clone();
            thread::spawn(move || {
                let cell = t.get_or(|| Cell::new(0));
                cell.set(cell.get() + 1);
                println!("{:?}", cell.get());
            }).join().unwrap();
        }

        let tls = Arc::try_unwrap(tls).unwrap();
        let total = tls.into_iter().fold(0, |x, y| x + y.get());
        println!("total {:?}", total);
        // println!("total2 {:?}", tls.get().unwrap().get());
    });


    // 多线程通信
    TestUtil::wrap_with_label("mpsc", || {
        let (tx, rx) = mpsc::channel();
        
        let txx = tx.clone();
        thread::spawn(move || {
            tx.send(String::from("123")).unwrap();
        });

        thread::spawn(move || {
            for i in 0..5 {
                txx.send(format!("666{:?}", i)).unwrap();
                thread::sleep(Duration::from_millis(100));
            }
        });

        // rx 支持迭代器
        for recived in rx {
            println!("recieved {}", recived);
        }
    });

    // 据说性能更好的通信库
    TestUtil::wrap_with_label("flume", || {
        let (send, recv) = flume::unbounded();

        for i in 0..5 {
            let s = send.clone();
            thread::spawn(move || {
                s.send(i).unwrap();
            });
        }

        // 需要手动 drop ，不然 recv 会持续等下去
        // 因为上面再线程中用的是 clone
        drop(send);

        let r: i32 = recv.iter().sum();
        println!("{:?}", r);
    });


    // mutex 锁
    TestUtil::wrap_with_label("mutex", || {
        let collector = Arc::new(Mutex::new(Vec::new()));
        let mut handlers = Vec::new();

        for i in 0..10 {
            let c = collector.clone();
            handlers.push(thread::spawn(move || {
                let mut r = c.lock().unwrap();
                r.push(i);
            }));
        }

        for h in handlers {
            h.join().unwrap();
        }

        println!("{:?}", collector.lock().unwrap());
    });

    // rw 锁
    TestUtil::wrap_with_label("rwlock", || {
        let collector = Arc::new(RwLock::new(Vec::new()));
        let c = collector.clone();
        thread::spawn(move || {
            // 允许一个写
            let mut w = c.write().unwrap();
            w.push(1);
            w.push(2);
        }).join().unwrap();

        thread::spawn(move || {
            // 允许多个读
            let r = collector.read().unwrap();
            let r2 = collector.read().unwrap();
            let r3 = collector.read().unwrap();

            println!("{:?}, {:?}, {:?}", r, r2, r3);
        }).join().unwrap();
    });

    // 裸指针实现 Send 和 Sync
    TestUtil::wrap_with_label("send_sync", || {
        #[derive(Debug)]
        struct MyBox(*mut i32);
        unsafe impl Send for MyBox {}
        unsafe impl Sync for MyBox {}

        let p = Arc::new(MyBox(5 as *mut i32));
        thread::spawn(move || {
            println!("{:?}", p.0);
        }).join().unwrap();
    });
}
