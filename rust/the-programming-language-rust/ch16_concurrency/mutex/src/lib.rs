// 16.3 공유 상태 동시성
// 채널 방식의 메시지 패싱은 단일 소유권과 유사하다.
// transmitter가 데이터를 전송하면 해당 데이터의 소유권은 receiver에게 넘어간다.

// 반면, 공유 메모리 동시성은 복수 소유권과 유사하다.
// 복수 개의 스레드들이 동시에 동일한 메모리 위치를 접근할 수 있다.
// 스마트 포인터들이 복수 소유권을 가능하게 만들어준다. (Rc)
// 복수 소유권은 이 서로 다른 소유자들의 관리가 필요하기 때문에 복잡성을 더할 수 있습니다.

// 공유 메모리를 위한 더 일반적인 동시성의 기초 재료 Mutex
// Mutex는 상호 배제(mutual exclusion)의 줄임말로서,
// 내부에서 뮤텍스는 주어진 시간에 오직 하나의 스레드만 데이터 접근을 허용합니다.

// 뮤텍스 내부의 데이터에 접근하기 위해서 스레드는 먼저 뮤텍스의 락(lock)을 얻기를
// 요청함으로써 접근을 원한다는 신호를 보내야 합니다.
// 뮤텍스는 잠금 시스템을 통해 가지고 있는 데이터를 보호하는 것으로 묘사됩니다.

// 뮤텍스의 두 가지 원칙
// 데이터를 사용하기 전에 반드시 락을 얻는 시도를 해야한다.
// 데이터 사용이 끝났다면, 반드시 언락(unlock)을 해야한다.

// Mutex<T>의 API
use std::mem::drop;
use std::sync::Mutex;
#[test]
fn test_mutex() {
    let m = Mutex::new(5);
    {
        // 반환값인 num:Mutex Guard는 스마틒 포인터로 가변 참조자 처럼 다룰 수 있다.
        let mut num = m.lock().unwrap();
        *num = 6
    } // Mutex Guard는 Drop 트레이트를 구현하여 자동으로 lock을 풀어준다.

    let mut num = m.lock().unwrap();
    *num = 7;
    drop(num); // 같은 scope에서 스마트 포인터를 생성했으므로, lock을 풀기 위해 drop 메서드 호출

    println!("m = {:?}", m);
}

// 여러 스레드들 사이에서 Mutex<T> 공유하기
// 이때, spawn한 스레드에서 실행할 클로저에 Mutex<T>의 소유권을 넘겨줘야 한다.
// Rc<T> 포인터를 사용하여 소유권을 공유할 수 있다.
// 그러나, Rc<T>는 스레드를 교차하면서 공유하기에는 안전하지 않다.
// 앞선 15장에서 Rc<T>는 싱글 스레드에서만 사용 가능하다고 언급하였다.

// 그 이유는, Rc<T>가 참조 카운트를 관리할 때, 각각의 clone 호출마다 카운트에 더하고
// 각 클론이 버려질 때마다 카운트에서 제한다.
// 그런데 다른 스레드에서 카운트를 바꿀만한 행동을 할 수 있으므로 참조 카운트를 안전하게 관리하기 어렵다.

// 해결책: Arc<T> (Automic Reference Counting)
// Arc<T>는 동시적 상황에서 안전하게 사용할 수 있는 Rc<T> 타입이다.
// Arc<T>는 Send, Sync 트레이트가 구현되어 있습니다.
// Send 트레이트는 스레드 사이에서 이전될 수 있음을 나타낸다.
// Sync 트레이트는 여러 스레드로 부터 안전하게 참조 가능함을 나타냅니다.

use std::rc::Rc;
use std::sync::Arc;
use std::thread;
#[test]
fn test_sharing_mutex_between_threads() {
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

    for h in handles {
        h.join().unwrap();
    }

    println!("Count: {}", *counter.lock().unwrap());
}
