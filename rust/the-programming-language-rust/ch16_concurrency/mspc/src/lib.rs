// 16.2 메시지 패싱을 사용하여 스레드 간에 데이터 전송하기
// 안전한 동시성을 보장하는 인기 상승중인 접근법 하나는 메시지 패싱이다.
// 이는 스레드들 혹은 액터들이 데이터를 담고 있는 메시지를 서로 주고 받는 것이다.

// 러스트가 메시지 보내기 방식의 동시성을 달성하기 위해 갖춘
// 한가지 중 도구는 '채널(channel)'이다.

// 프로그래밍에서 채널은 개울이나 강 같은 물의 통로와 비슷하다고 상상할 수 있다.
// 채널은 크게 송신자(transmitter)와 수신자(receiver)로 나눠져 있다.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
#[test]
fn test_msg_passing_channel() {
    // mpsc는 복수 생성자(multiple producer), 단일 소비자(single consumer)를 의미한다
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // 스레드에서 tx(transmitter) 변수 소유
                               // send 메서드는 매개변수의 소유권을 수신자에게 이전시킴
                               // 따라서 val 변수는 해당 스레드에서 다시 사용할 수 없음
    });

    // recv 메서드는 메시지가 올 떄 까지 기다림
    let received = rx.recv().unwrap();
    //let received = rx.try_recv().unwrap(); try_recv 메서드는 실행 시점에서 메시지가 도착했는지 확인

    println!("Got: {}", received);
}

// 복수의 값들을 보내고 수신자가 기다리는지 보기
#[test]
fn test_send_multiple_msgs() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msgs = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("transmitter"),
        ];

        for m in msgs {
            tx.send(m).unwrap();
            thread::sleep(Duration::from_secs(1))
        }
    });

    // recv 메서드를 명시적으로 호출하지 않고, 수신자를 Iterator 처럼 다룰 수 있다.
    for m in rx {
        println!("Got: {}", m);
    }
}

#[test]
fn test_multiple_transmitter() {
    let send_closure = |tx: mpsc::Sender<String>| {
        // curryied closure
        move || {
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
        }
    };

    let (tx, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);

    //send_closure(tx);
    thread::spawn(send_closure(tx));
    thread::spawn(send_closure(tx2));

    for m in rx {
        println!("Got: {}", m);
    }
}
