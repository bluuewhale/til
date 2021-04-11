// 19.1 안전하지 않은 러스트
// 현재까지 작성한 코드들은 컴파일 타임에 강제되는 러스트의 메모리 안전성 보장을 갖는다.
// 그러나, 러스트는 이러한 메모리 안전성 보장을 강제하지 않는 숨겨진 내부의 두번째 언어를 갖는다.
// 이것을 안전하지 않은 러스트(unsafe Rust)라고 부른다

// 19.1.1 안전하지 않은 슈퍼파워
// 안전하지 않은 러스트로 전환하기 위해서는 unsafe 키워드를 사용한다.
// 그 다음 안전하지 않은 코드를 감싸주는 새 블록을 시작한다.

// 안전하지 않은 러스트에서 제공하는 4가지 기능
// 1. 로우 포인터(raw pointer)를 역참조하기
// 2. 안전하지 않은 함수 혹은 메소드 호출하기
// 3. 가변 정적 변수(mutable static variable)의 접근 혹은 수정하기
// 4. 안전하지 않은 트레잇 구현하기

// unsafe가 대여 검사기나, 러스트의 안전성 검사 기능을 끄는 것은 아니다.
// 단지, 위에 언급된 4개의 기능을 추가적으로 허용해주는 것이다.

// 19.1.2 로우 포인터를 역참조하기
// 러스트에서는 참조자들이 언제나 유효함을 컴파일러가 보장한다.
// 안전하지 않은 러스트는 로우 포인터(raw pointer)라고 불리는 참조자와 유사한 두 가지 새로운 타입을 갖습니다.
// 참조자를 이용하는 것처럼 로우 포인터도 불변 혹은 가변이 될 수 있으며,
// 각각 *const T(불변 로우 포인터)와 *mut T(가변 로우 포인터)라고 씁니다.

// 참조자나 스마트 포인터와는 다르게, 아래와 같은 로우 포인터의 성질을 명심하세요.
// 로우 포인터는 빌림 규칙 무시가 허용되어, 불변 및 가변 로우 포인터를 동시에 가질 수 있다.
// 로우 포인터는 유효한 메모리를 가리키고 있음을 보장하지 않는다.
// 로우 포인터는 널이 될 수 있다.
// 로우 포인터는 자동 메모리 정리가 구현되어 있지 않다.

#[test]
fn test_create_raw_pointer() {
    let mut num = 5;

    // 이미 유효성이 보장된 참조자로부터 직접 로우 포인터를 만들었다.
    // 따라서, 이러한 로우 포인터는 유효성이 보장된다
    // 로우 포인터는 불변 참조와 가변 참조를 동시에 생성했음에도 컴파일이 된다. (대여 규칙 무시)
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 안전한 코드 내에서는 로우 포인터를 역참조하여 메모리에 저장된 데이터를 읽을 수 없다.
    // 따라서, 로우 포인터를 역참조할 때는, 항상 unsafe 영역 내에서만 가능하다.
    unsafe {
        println!("r1 is : {}", r1);
        println!("r2 is : {}", r2)
    }
}

#[test]
fn test_create_unsafe_raw_pointer() {
    let address = 0x012345usize;
    let r = address as *const i32; // 임의의 메모리 주소를 가르키는 로우 포인터
}

// 로우 포인터를 왜 사용할까?
// 1. C코드와의 상호작용을 할 때
// 2. 대여 검사기가 이해하지 못하는 안전한 추상화를 만들 때

// 안전하지 않은 함수 혹은 메서드 호출하기
#[test]
fn test_create_unsafe_func() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

// 안전하지 않은 코드 상에 안전한 추상화 생성하기
// split_at_mut 메서드는 사실 내부적으로 안전하지 않은 코드를 담고 있다.
// 가변 참조를 동시에 2개나 생성하기 때문에 안전한 러스트에서는 컴파일이 되지 않는다.
#[test]
fn test_split_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // as_mut_ptr()은 *mut i32 로우 포인터를 반환
    assert!(mid <= len);

    unsafe {
        (
            // from_raw_parts_mut 메서드는 로우 포인터를 인자로 사용하고
            // 이 포인터가 유효함을 반드시 믿어야 하므로 안전하지 않습니다.
            // offset 메서드 또한, 안전하지 않다.
            // 하지만 위에서 수행한 assert!(mid <= len) 검사로 인해 안전서이 보장됨
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

// extern 함수를 사용하여 외부 코드 호출하기
#[test]
fn test_extrn() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 가변 정적 변수의 접근 혹은 수정하기
// 안전한 러스트는 소유권 규칙으로 인해 가변 전역 변수를 다루기 어렵다.
// 두 스레드가 동일한 가변 전역변수에 접근하면 데이터 레이스를 야기할 수 있다.
// 러스트에서 전역 변수는 정적(static) 변수라고 불립니다.

static HELLO_WORLD: &str = "Hello, world!";
#[test]
fn test_static_variable() {
    println!("{}", HELLO_WORLD);
}
// 정적 변수는 상수와 유사하지만 조금 다른 특징을 갖는다.
// 먼저, 정적 변수는 SCREAMING_SNAKE_CASE 형식을 따르며, 반드시 변수의 타입을 명시해야한다.
// 또한, 정적 변수는 'static 수명을 갖는다.
// 정적 변수는 항상 메모리 내의 고정된 주소 값을 갖는 반면, 상수는 사용될 때마다 데이터가 복사되는 것을 허용한다.

// 상수와 정적 변수 간의 또다른 차이점은 정적 변수가 가변일 수 있다는 점이다.
// 가변 정적 변수에 접근하고 수정하는 것은 '안전하지 않다'.

fn test_mut_static_var() {
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);

    unsafe {
        println!("Counter: {}", COUNTER);
    }
}

// 안전하지 않은 트레잇 구현하기
// unsafe에서만 동작하는 마지막 기능은 안전하지 않은 트레잇 구현이다.
// 트레잇은 적어도 메소드 중 하나가 컴파일러가 검사할 수 없는
// 몇몇 불변성(invariant)을 갖고 있을 때 안전하지 않다.

// Sync와 Send 마커 트레잇을 상기해보자
// 우리의 타입이 전체적으로 Send되고, Sync한 타입으로 구성되어 있다면
// 컴파일러는 이 트레잇을 자동적으로 구현한다
// 만일 우리가 로우 포인터와 같이, Send되지 않거나 Sync하지 않은 타입을 포함한 타입을 구현한다면
// unsafe를 활용해야 한다.
