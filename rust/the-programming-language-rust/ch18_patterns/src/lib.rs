// 18. 패턴과 매칭

// 패턴은 단순하거나 복잡한 타입의 구조에 값들을 비교하기 위한 러스트의 특별한 문법입니다.
// 패턴을 match 표현 및 다른 구문들과 함께 사용하면 프로그램 흐름을 더 많이 제어할 수 있습니다.

// 18.1 패턴이 사용될 수 있는 모든 곳

// 18.1.1 match 갈래
// match 표현을 사용하기 위해서는 match 표현이 대응 시킬 값이 가질 수 있는
// 모든 경우의 수를 빠짐 없이 표현해야 한다.
// _라는 특별한 패턴은 아무 겂에나 대응되지만 그 어떤 변수에도 묶이지 않는다.

/*
match 값 {
    패턴 => 표현,
    패턴 => 표현,
    _ => 표현,
}
*/

// 18.1.2 if let 조건 표현
// if let은 match와 달리 모든 경우의 수를 매칭하지 않아도 된다.
#[test]
fn test_if_let() {
    let fav_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = fav_color {
        println!("Favorite Color: {}", color);
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("purple");
        } else {
            println!("orange");
        }
    } else {
        println!("blue");
    }
}

#[test]
fn test_while_let() {
    let mut stack = vec![];
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(v) = stack.pop() {
        println!("{}", v);
    }
}

#[test]
fn test_for_loop() {
    let values = vec!['a', 'b', 'c'];

    for (idx, v) in values.iter().enumerate() {
        println!("{} {}", idx, v);
    }
}

// let 구문
// 사실 let 구문은 패턴이 쓰이고 있다
// let PATTERN = EXPRESSION;
#[test]
fn test_let_pattern() {
    let (x, y, z) = (1, 2, 3);
}

#[test]
fn test_func_arg_pattern() {
    fn print_point(&(x, y): &(i32, i32)) {
        println!("Current Location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_point(&point);
}

// 18.2 반증 가능성(Refutability): 패턴이 매칭에 실패할지의 여부
// 패턴은 2가지 형태가 존재합니다.
// 반증 가능 패턴(실패할 수 있는 패턴) & 반증 불가 패턴(실패할 수 없는 패턴)
// 함수의 매개변수, let 구문, for loop 구문은 반증 불가 패턴만 사용 가능하다

// 18.3 패턴 문법의 모든 것

// 18.3.1 리터럴 매칭
#[test]
fn test_literal_matching() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// 18.3.3 다중 패턴
#[test]
fn test_multiple_pattern() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

// 18.3.4 ...을 이용한 범위 매칭
#[test]
fn test_range_matching() {
    let x = 5;

    match x {
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'...'j' => println!("early ASCII letter"),
        'k'...'z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// 값을 해체하여 분리하기
// 구조체 해체하기
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_decompose_struct() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("point is on x axis"),
        Point { x: 0, y } => println!("point is on y axis"),
        Point { x, y } => println!("on neither axis: ({} {})", x, y),
    }
}

// 열거형 해체
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[test]
fn test_enum_matching() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => println!("Quit!"),
        Message::Move { x, y } => println!("move to ({}, {})", x, y),
        Message::Write(text) => println!("Text Message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: {} {} {}", r, g, b),
    }
}

// 참조자 해체
#[test]
fn test_ref_decomposition() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
}

// 패턴 내에서 값 무시하기
// _를 이용해 전체 값 무시하기
#[test]
fn test_ignore() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);
}

// 중첩된 _를 이용해 값의 일부 무시하기
#[test]
fn test_ignore_with_multiple_() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
}

// ..를 이용해 나머지 부분 무시하기
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

#[test]
fn test_dot_dot() {
    let point = Point3D { x: 1, y: 2, z: 3 };

    match point {
        Point3D { x, y, z } => println!("({}, _, {})", x, z),
    }
}

// re와 ref mut를 이용해 패턴 내에서 참조자 생성하기
// ref를 사용해 참조자를 만들어서 패턴 내 변수로 값의 소유권이 이동하지 않도록 설정 가능

#[test]
fn test_ref() {
    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref name) => println!("My name is {}", name),
        None => (),
    }

    println!("Robot {:?} still has its ownership", robot_name);
}

#[test]
fn test_ref_mut() {
    let mut robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }

    println!("Robot name has changed to {:?}", robot_name);
}

// 매치 가드를 이용한 추가 조건
// Type 검사 이후, value에 대한 검사를 추가적으로 수행
#[test]
fn test_match_guard() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five"),
        Some(x) => println!("greater than four"),
        None => (),
    }
}

// @ 바인딩
// at 연산자인 @는 해당 값이 패턴과 매치되는지 확인하는 동시에 해당 값을 갖는 변수를
// 생성할 수 있도록 해줍니다.

#[test]
fn test_at_binding() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };
    // Type 체크 이후, 필드 값에 대한 검사를 수행할 수 있도록 해주는 연산자
    match msg {
        Message::Hello {
            id: id_variable @ 3...7, // 범위에 대한 검사 이후, 값을 capture한다.
        } => {
            println!("Found an id in range: {}", id_variable);
        }
        Message::Hello { id: 10...12 } => {
            println!("Found an id in range 10 ~ 11");
        }
        Message::Hello { id } => println!("Found an id: {}", id),
    }
}
