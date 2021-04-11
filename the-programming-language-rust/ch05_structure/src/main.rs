fn main() {
    test_structure();
    test_tuple_structure();
}

fn test_structure() {
    let mut user1 = User {
        username: String::from("user001"),
        email: String::from("user001@gmail.com"),
        sign_in_count: 1,
        is_active: true,
    };
    // 구조체 인스턴스 자체가 가변임
    user1.email = String::from("user001@naver.com");

    let user2 = build_user(String::from("user002"), String::from("user002@gmail.com"));
    let user3 = build_user_from_another_user_instance(
        String::from("user003"),
        String::from("user003@gmail.com"),
        user2,
    );

    // 5.2 구조체를 사용한 예제 프로그램
    let rectangle = Rectangle {
        width: 30,
        height: 20,
    };
    println!("면적: {}", calc_area(&rectangle)); // 참조를 사용함으로써, main 함수 scope에 rectangle의 소유권을 유지시킴

    // 5.2.3 트레이트를 상속해서 유용한 기능 추가하기
    let rectangle = Rectangle2 {
        width: 30,
        height: 20,
    };
    println!("Structure: {:#?}", rectangle);

    // 5.3 매서드 문법
    let rectangle = Rectangle {
        width: 30,
        height: 20,
    };
    println!("area(): {}", rectangle.area());

    // 5.3.2 더 많은 매개변수를 갖는 매서드
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rec1은 rec2를 포함하는가? {}", rec1.can_hold(&rec2));
    println!("rec1은 rec3를 포함하는가? {}", rec1.can_hold(&rec3));

    // 5.3.3 연관 함수 (staticmethod)
    let squ1 = Rectangle::square(32);
    println!("{:#?}", squ1);
}

// Structure
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn build_user(username: String, email: String) -> User {
    return User {
        username,
        email,
        sign_in_count: 1,
        is_active: true,
    };
}

fn build_user_from_another_user_instance(username: String, email: String, user: User) -> User {
    return User {
        username,
        email,
        ..user
    };
}

// Tuple Structure
// 필드에 값만 있고 이름은 없는 구조체
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn test_tuple_structure() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Unit Structure
// 필드가 하나도 없는 구조체

// 5.2 구조체를 사용하는 예제 프로그램
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn calc_area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

// 5.2.3 트레이트를 상속해서 유용한 기능 추가하기
#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

// 5.3 매서드 문법
impl Rectangle {
    fn area(&self) -> u32 {
        // 자신에 대한 불변 참조를 호출 (값을 바꾸지 않고, 소유권도 그대로 유지하기 떄문)
        return self.width * self.height;
    }
}

// 5.3.2 더 많은 매개변수를 갖는 매서드
impl Rectangle {
    fn can_hold(&self, rec: &Rectangle) -> bool {
        if (self.width > rec.width) && (self.height > rec.height) {
            return true;
        }
        return false;
    }
}

// 5.3.3 연관 함수 (static method)
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }
}
