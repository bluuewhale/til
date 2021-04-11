// 8.1 벡터에 일련의 값 저장하기

// 8.1.1 새로운 백터 생성
// 백터는 제네릭을 이용하여 구현되었기 때문에 어떤 데이터 타입이든 저장할 수 있다.
// vec! 매크로를 사용하면 편리하게 벡터를 생성 가능
pub fn create_vector() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
}

// 8.1.2 백터에 값 추가
pub fn update_vector() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

// 8.1.3 백터 해제
pub fn drop_vector() {
    let v = vec![1, 2, 3, 4];
    // do something with v
} // <- 변수 v가 scope을 벗어났으므로, drop 매서드가 호출되어 메모리가 해제됨

// 8.1.4 백터로부터 값 읽기
pub fn read_vector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2]; // 인덱스 문법의 참조로 값 읽기, 인덱스 문법은 해당 인덱스가 존재하지 않으면 panic이 발생
    println!("세 번째 원소: {}", third);

    match v.get(2) {
        // get 매서드는 Option<&T> 타입의 값을 리턴한다. 따라서, 백터에 해당 인덱스 값이 없으면 None을 return
        Some(value) => println!("세 번째 원소: {}", value),
        None => println!("세 번째 원소가 존재하지 않습니다."),
    }
}

pub fn test_push_with_reference() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // 불변 참조 생성

    // 동일한 범위 내에서, 불변 참조와 가변 참조를 동시에 가질 수 없다.
    // 이미 third 변수가 v 벡터를 불변참조 하고 있으므로, 아래의 코드는 컴파일 에러 발생
    // ! 정상 작동하는데 원인을 모르겠음..
    v.push(6);
}

// 8.1.5 벡터에 저장된 값을 순회하기
pub fn iterate_vector() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &v {
        // 불변 참조 순회
        println!("{}", i);
    }

    for i in &mut v {
        // 가변 참조 순회
        *i += 50; // 가변 참조가 가르키는 값을 수정하려면 역참조 연산자(*)가 필요
    }

    for i in &v {
        println!("{}", i);
    }
}

// 8.1.6 열거자를 이용해 여러 타입 저장하기
// 벡터는 동일한 타입의 데이터만 저장 가능
// 따라서, 다양한 데이터 타입을 두루고 싶은 경우에는 열거자가 유용하게 활용됨
pub fn vector_with_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("블루")),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}
