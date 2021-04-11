// 러스트는 중복의 개념을 효과적으로 다루기 위해 generic을 사용
// 제네릭은 구체적인 타입(concrete type)이나 다른 속성을 대체할 수 있는 추상화된 타입을 활용한다.

// 10.1 함수로부터 중복 제거하기
pub fn run_without_generic() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for n in number_list {
        if n > largest {
            largest = n;
        }
    }
    println!("가장 큰 숫자: {}", largest);
}

// 제네릭을 사용하지 않는 경우 타입별로 함수를 정의해야 함
pub fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

pub fn largest_u32(list: &[u32]) -> u32 {
    let mut largest = list[0];

    for &n in list.iter() {
        if n > largest {
            largest = n
        }
    }

    return largest;
}

pub fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &s in list.iter() {
        if s > largest {
            largest = s;
        }
    }

    return largest;
}

// 10.2 제네릭 데이터 타입
// 일반적으로 제네릭을 사용하는 함수를 정의할 때는
// 특정한 타입의 매개변수와 리턴 타입을 사용하는 함수의 시그니처에 적용

// 아래의 코드는 아직 컴파일되지 않는다.
// 비교 연산(>)을 수행할 타입들은 std::cmp::PartialOrd 트레이트를 구현할 것을 요구하기 때문이다.
/* pub fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}
*/

// 10.2.2 구조체 정의에서 제네릭 사용하기
pub struct Point<T> {
    x: T,
    y: T,
}
pub fn create_struct_with_generic_type() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
}

// 10.2.3 열거자 정의에서 제네릭 사용하기
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 코드에서 여러 개의 구조체나 열거자가 오직 저장하는 값의 타입만 다를 때는
// 제네릭 타입을 활용하여 중복을 효과적으로 제거할 수 있다.

// 10.2.4 메서드 정의에서 제네릭 사용하기
struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Point2<T> {
    // generic 타입에 대한 impl
    fn x(&self) -> &T {
        return &self.x;
    }
}

impl Point2<f32> {
    // 구체화된 타입에 대한 impl
    fn distance_from_origin(&self) -> f32 {
        return (self.x.powi(2) + self.y.powi(2)).sqrt();
    }
}

// 구조체의 정의와는 다른 제네릭 타입을 사용하는 메서드
pub struct Point3<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point3<T, U> {
    fn mixup<V, W>(self, other: Point3<V, W>) -> Point3<T, W> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn mixup_generic() {
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2); // p1 소유권 상실
    println!("p3.x = {}, p3.y= {}", p3.x, p3.y);
}

// 10.2.5 제네릭의 성능
// 제네릭 타입 매개변수를 사용해도 런타임에 성능 저하가 거의 발생하지 않는다.
// 이유는 러스트가 컴파일 시점에 제네릭을 사용하는 코드를 '단일화(Monomorphzation)'하기 때문이다.
// 단일화란, 컴파일 시점에 제네릭 코드를 실제로 사용하는 구체화된 타입으로 변환하는 과정이다.

pub fn monomorphzate() {
    let integer = Some(5);
    let float = Some(5.0);

    // after monomorphzation
    // 컴파일러는 단일화를 수행할 때, 제네릭의 정의를 각 타입별로 확장한다.
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
