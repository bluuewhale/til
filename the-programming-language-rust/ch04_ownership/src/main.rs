// Rust에서는 가비지 콜렉터에 의존하지 않고도 메모리 안정성을 보장하려는 Ownership이라는 개념을 갖고 있다.
// Rust에서는 데이터가 어떤 메모리(heap or stack)에 저장되어 있는지에 따라서 언어의 동장이나 의사결정이 크게 달라진다.

// 스택에 저장되어 있는 데이터는 모두 고정된 크기를 가져야한다.
// 띠라서, 컴파일 시점에 크기를 알 수 없는 데이터나 런타임에 동적으로 크기가 변하는 데이터는 힙 메모리에 저장된다.

// 운영체제는 메모리에서 빈 공간을 찾아서 데이터에 메모리를 할당(allocate)하고 포인터(메모리 주소를 가르키는 객체)를 반환한다.
// 반면, Stack 메모리에 올라간 데이터에서는 할당 작업이 필요하지 않다 ==> 더 빠름

// 코드에서 함수를 호출할 때는 매개변수들이 로컬 변수에 할당되어 스택에 저장되고, 함수 호출이 끝나면 스택에서 제거됨

// Rust의 Ownership은 힙에 저장되는 데이터의 중복을 최소하하고 힙 메모리를 최적화하기 위한 개념

// 소유권 규칙
// 러스트가 다루는 각각의 값은 소유자(owner)라고 부르는 변수를 가지고 있다.
// 특정 시점에 값의 소유자는 단 하나 뿐이다.
// 소유자가 범위(Scope)를 벗어나면 그 값은 제거된다.

fn main() {
    create_string();
    _move();
    clone();
    copy();
    ownership();

    slice();
}

fn create_string() {
// 앞서 살펴본 데이터 타입들은 모두 스택에 저장되며 범위를 벗어나면 스택에서 제거됨
    // 반면 이제부터 살펴본 데이터 타입들은 힙에 저장되며 데이터가 제거되는 시점이 다름

    // String Type
    // 다음과 같은 문자열 리터럴의 단점을 개선하기 위한 데이터 타입
    // - immutable
    // - 동적 할당 불가 (컴파일 시점에서 길이를 알 수 없어서, 바이너리 형태로 미리 변환할 수 없음)
    
    let mut s = String::from("Hello"); // heap 메모리 공간 요청
    s.push_str(", String!"); // push_str() 매서드는 String 인스턴스에 문자열 리터럴을 결합한다.

    println!("{}", s);

} // fn_string 함수의 범위를 벗어난 시점에서 s는 더 이상 유효하지 않음 
// 일반적인 프로그래밍 언어에서는 GC가 사용하지 않는 메모리를 추적해 해제한다.
// 러스트는 닫는 중괄호를 만나면 자동으로 drop 함수를 호출해서 자원을 해제함 => RAII

fn _move(){
    // 변수와 데이터가 상호작용하는 방식

    // 리터럴 타입은 deep copy
    // 그 외에는 shallow copy (포인터, 길이, 용량만 복사)
    
    let s1 = String::from("Let's Move!");
    let s2 = s1; // Rust는 shallow copy가 발생하면 이전 변수를 무효화 시켜버림 (move) => s1에 접근 불가

    // 따라서, 아래 식은 에러를 발생시킴
    // println!("{}", s1); 
    println!("{}", s2);

}

fn clone() {
    // 힙 메모리에 저장된 데이터를 deep copy하려면 clone 매서드를 사용
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("Let's clone {}, {}", s1, s2);
}

fn copy() {
    // 스택에 저장되는 데이터 deep copy와 shallow copy에 차이가 없다
    // 따라서 clone() 매서드를 호출하지 않았음에도 유효
    let x = 5;
    let y = x;
    println!("{} {}", x, y);
    // 러스트에서는 스택에 저장되는 데이터 타입에 Copy trait를 제공한다.
    // Copy trait가 적용된 경우, 이전 변수를 새 변수에 할당해도 무효화 되지 않는다.
    // Copy trait는 스택에 저장되는 리터럴 타입 자료형들에 적용되어 있다.
    
    // u32와 같은 모든 정수형 타입
    // bool
    // char
    // f64와 같은 모든 부종 소수점 타입
    // Copy trait가 적용된 타입을 포함하는 튜플
}

fn ownership() {
    // 값을 함수에 전달한다는 의미는 값을 변수에 대입하는 것과 유사하다.
    // 따라서, 값의 이동이나 복사가 이루어진다.

    let s = String::from("Ownership");
    take_ownership(s); // 위 함수에서 s에 대한 ownership을 가져감. 이후 s는 함수의 scope을 벗어나서 메모리 해제됨
    
    // 이후로, s는 더 이상 유효하지 않기 때문에 아래 구문은 "use of moved value" 에러를 발생시킴
    // take_ownership(s);

    // 그러나 참조를 활용하면 기존 변수의 소유권은 그대로 유지됨
    let s2 = String::from("Ownership!");
    borrow_ownership(&s2);
    borrow_ownership(&s2);

    // 참조는 기본적으로 불변이지만, 가변 참조를 활용하면 수정 가능
    // 동일한 scope내에 불변/가변 참조가 하나도 없는 경우에만 가변 참조를 생성할 수 있음
    let mut s3 = String::from("Am I changed?");
    borrow_ownership_and_add_change(&mut s3)
}

fn take_ownership(s: String) {    // ownership moved
    println!("{} Moved!", s);
}

fn borrow_ownership(s: &String) { // 참조 변수를 받는 경우, 기존 변수의 ownership이 이전되지 않음
    println!("{} has been borrowed !", s);
}

fn borrow_ownership_and_add_change(s: &mut String){
    s.push_str(" Yes!");
    println!("{}", s);
}

fn _check_dead_ownership(){
    // 러스트에서는 컴파일러가 죽은 참조가 발생하지 않도록 보장해준다.
    // 어떤 데이터에 대한 참조를 생성하면 컴파일러가 참조를 실행하기에 앞서 데이터가 범위를 벗어나지 않았는지 확인해줌

    let _dead_s = String::from("Do not reference me!");

    // 아래의 참조를 반환하면 lifetime 에러 (참조해오는 변수가 이미 메모리에서 해제되었음)
    // &dead_s  

    // 따라서 참조가 아닌 String 타입을 직접 리턴하여야 함 (Ownership Move)
}

fn slice(){
    let s = String::from("Slice me");
    let first_word = slice_first_word(&s);

    println!("{}", first_word);

    let my_string = String::from("my string");
    let first_word = slice_first_word_enhanced(&s[..]);
    
    let my_str_literal = "my str literal";
    let first_word2 = slice_first_word_enhanced(&my_str_literal);
}

fn slice_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
            }
        }

    return &s[..]
}

fn slice_first_word_enhanced(s: &str) -> &str {
    // 문자열 리터럴은 바이너리에 저장되는 불변 값
    // 따라서, 문자열 리터럴에 대한 참조는 불변 참조이며, 문자열 리터럴은 이미 문자열 슬라이스이다.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
            }
        }

    return &s[..]
}
