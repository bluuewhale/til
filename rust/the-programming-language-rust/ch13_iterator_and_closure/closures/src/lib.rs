// 13.1 클로저: 주변 환경을 캡처하는 익명 함수
// 클로저는 변수에 저장하거나 다른 함수에 인자로 전달하는 익명 함수이다.
// 일반 함수와 달리 클로저는 자신이 정의된 범위 내의 값들을 캡처(capture)한다.

// 13.1.1 클로저를 이용한 동작의 추상화
// 나중에 실행할 클로저를 저장하는 상황
// python의 lambda와 유사한 개념 같다.

use std::thread;
use std::time::Duration;

pub fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("시간이 오래 걸리는 계산을 수행하는 중...");
    thread::sleep(Duration::from_secs(2));
    return intensity;
}

pub fn generate_workout(intensity: u32, salt: u32) {
    let expensive_closure = |intensity: u32| -> u32 {
        println!("시간이 오래 걸리는 계산을 수행하는 중...");
        thread::sleep(Duration::from_secs(2));
        return intensity;
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if salt == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}
// 13.1.3 제네릭 매개변수와 Fn 트레이트를 이용해 클로저 저장하기
// 클로저와 클로저의 결과 값을 캐싱할 구조체를 선언한다.
// 클로저 + 트레이트 경계 + 제네릭 타입 + 옵션 열거자

// Cacher의 한계
// 최초에 value로 받은 값에 대한 결과만 저장함 => 해시맵을 사용
// u32 타입에 대해서만 동작 => 제네릭 데이터 타입 사용

struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                return v;
            }
        }
    }
}

pub fn generate_workout_enhanced(intensity: u32, salt: u32) {
    let mut expensive_closure = Cacher::new(|intensity| {
        println!("시간이 오래 걸리는 계산을 수행하는 중...");
        thread::sleep(Duration::from_secs(2));
        return intensity;
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if salt == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

// 13.1.5 클로저를 이용해 주변 환경 캡처하기
// 클로저는 함수에는 업슨 독특한 기능을 제공한다.
// 바로 자신이 정의된 범위 내의 환경(변수 등)을 캡처해서 메모리에 기록한다.
// 따라서, 환경을 저장할 필요가 없는 경우에는 오버헤드가 적은 함수를 사용하자.

pub fn test_capture() {
    let x = 4;
    let equal_to_x = |y| y == x;

    assert!(equal_to_x(4));
}

// 클로저는 크게 세가지 방법(소유권 획득, 불변 참조, 가변 참조)으로 환경을 가져온다.
// FnOnce: 소유권 획득 => 클로저를 선언하는 시점에서 변수의 소유권을 가져간다. (move 키워드 사용)
// FnMut: 가변 참조 트레이트
// Fn: 불변 참조 트레이트

pub fn test_fnonce() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |y: Vec<u32>| y == x;

    // 변수 x의 소유권이 이전되었으므로 아래의 명령어는 컴파일 에러 발생
    // println!("{:?}", x);
}
