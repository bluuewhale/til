fn main() {
    // Scalar Type
    // 하나의 값을 저정하는 타입

    // 불변성, shadowing
    let _x = 5;
    println!("x = {}", _x);

    let _x = _x + 1;
    println!("x = {}", _x);

    let _x = _x * 2;
    println!("x = {}", _x);

    let _tab = "    "; // 4 spaces
    let _tab_size = _tab.len();
    println!("tab size: {}", _tab_size);
    
    // 변수형
    let _float = 2.0; // f64
    let _float2: f32 = 2.0; // f32(type annotation)

    // 사칙연산
    // 덧셈
    let _sum = 5 + 10;

    // 뺠셈
    let _difference = 95.5 - 4.3;

    // 곱셈
    let _product = 4 * 30;

    // 나눗셈
    let _quotient  = 56.7 / 32.2;

    // 나머지
    let _remainder = 43 % 5;

    // 불리언
    let _boolean: bool = true;

    // 문자 타입 (char)
    // 4바이트 유니코드 스칼라 값 (ASCII 보다 많은 문자를 표현할 수 있음)
    let _c = "z";
    let _z = "Z";


    // Compound Type
    // 여러 개의 값을 그룹화한 타입
    // Rust에서는 기본적으로 tuple과 array를 지원

    // Tuple
    // 서로 다른 타입의 여러 값을 하나의 컴파운드 타입으로 그룹화하기에 적합한 타입
    let _tup: (i32, f64, u8, char, bool) = (500, 6.4, 1, 'd', true);
    let (_a, _b, _c, _d, _f) = _tup; // 패턴 매칭을 이용한 destruct

    let _five_hundred = _tup.0; // inde_xing
    let _si_x_point_four = _tup.1;
    let _one = _tup.2;


    // Arary
    // 배열의 각 요소는 바드시 같은 타입이어야 한다
    // 고정된 길이를 갖는다.
    // 다루고자 하는 데이터를 힙(heap)이 아닌 스택(stack) 메모리에 할당할 때 유용
    // Vector 만큼 유연하지는 않음: Vector는 표준 라이브러리가 지원하며 배열과 유사하지만, 크기를 자유롭게 조정할 수 있다.
    let mut _arr: [i32; 5] = [1, 2, 3, 4, 5]; // [type, len]
    let _month = [
        "January", 
        "Feburary", 
        "March",
        "April",
        "May", 
        "June", 
        "July", 
        "August", 
        "September", 
        "October", 
        "November", 
        "December"
    ];

    // Array Inde_xing
    let _first_element = _arr[0];
    _arr[1] = 10; // can assign value, if array is declared as mu_table
    println!("{}", _arr[1]);

    // 아래와 경우, 컴파일 단계에서는 문제가 없지만, Rust에서는 런타임에서 에러(panic)를 발생시킴
    // 다른 저수준 언어에서는 이런 종류의 검사를 수행하지 않으며, 엉뚱한 메모리 값을 읽게 될 수도 있다.
    let index = 10;
    let _element = _arr[index]; 

    println!("{}", _element);
}
