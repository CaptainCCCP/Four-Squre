use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    //新建hashmap
    let mut goods = HashMap::new();

    goods.insert(String::from("wheats"), 50);
    goods.insert(String::from("clothes"), 10);

    let good_name = String::from("wheats");
    let good_number = goods.get(&good_name).copied().unwrap_or(0);
    //这里，score 是与蓝队分数相关的值，应为 10。get 方法返回 Option<&V>，
    //如果某个键在哈希 map 中没有对应的值，get 会返回 None。
    //程序中通过调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>，
    //接着调用 unwrap_or 在 score 中没有该键所对应的项时将其设置为零。
    for (key, value) in &goods {
        println!("{key}: {value}");
    }


    //channel返回发送者（transmitter）和 接收者（receiver）
    let (tx, rx) = mpsc::channel();
    //建立线程并发送信息
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
