// 16.1 코드를 동시에 실행하기 위한 쓰레드

// 요즘 사용하는 대부분 운영체제는 프로그램의 코드를 프로세스(process)로 실행하며,
// 운영체제가 한 번에 여러 개의 프로세스를 관리한다.
// 프로그램 안에서는 동시에 실행되는 독립된 부분이 있을 수 있다.
// 이렇게 독립적인 부분을 실행하는 기능을 스레드라고 한다.

// 스레드는 동시에 실행되므로, 본질적으로 다른 스레드에서 실행되는 코드의 순서를 보장할 수 없다.
// 경합 상태(race conditions) : 스레드가 일정하지 않은 순서로 데이터나 자원에 접근하는 상황
// 데드락(deadlocks): 두 스레드가 모두 서로가 자원의 사용을 마칠 때 까지 대기해서 두 스레드 모두 대기 상태에 놓이는 상황

// 많은 운영체제가 새로운 스레드를 생성하는 API를 지원한다.
// 프로그래밍 언어가 운영체제의 API를 호출해서 스레드를 생성하는 모델을 1:1 모델이라고 부른다.

// 많은 프로그래밍 언어는 스레드에 대해 독자적인 구현을 제공한다.
// 프로그래밍 언어가 제공하는 스레드는 그린 스레드(green threads)라고 하며,
// 언어는 그린 스레드를 이용해 운영체제 스레드와는 다른 수의 스레드를 실행한다.

// 러스트는 1:1 스레드 구현만을 지원한다.

// 16.1.1 spawn 함수로 새 스레드 생성하기
// 새로운 스레드를 생성하기 위해서는 thread::spawn 함수를 호출하고,
// 스래드 내에서 실행하기를 원하는 코드가 담겨 이는 클로저를 넘깁니다.

use std::thread;
use std::time::Duration;

#[test]
fn test_create_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// 16.1.2 join 핸들을 사용하여 모든 스레드들이 끝날때까지 기다리기
// thread::spawn은 JoinHandle 타입을 반환
// JoinHandle은 join 메서드를 호출했을 때, 그 스레드가 끝날 때까지 기다린다
#[test]
fn test_join_handle() {
    let handle = thread::spawn(|| {
        for i in 1..20 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

// 16.1.3 스레드에 move 클로저 사용하기
// move 클로저는 thread::spawn와 함께 자주 사용되는데 그 이유는
// 이것이 여러분으로 하여금 어떤 스레드의 데이터를 다른 스래드 내에서 사용하도록 해주기 때문입니다.
// move 클로저는 변수를 소유하므로, 메인 스레드와 독립적으로 변수를 제어할 수 있음

#[test]
fn test_thread_with_move_closure() {
    let v = vec![1, 2, 3];
    let closure = move || println!("Here's a vector : {:?}", v);

    let handle = thread::spawn(closure);
    handle.join().unwrap();
}
