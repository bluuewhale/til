// 8.2 String 타입에 UTF-8 형식의 텍스트 저장하기
// 문자열을 컬렉션의 일종으로 생각하는 것은 유용한 시각
// 러스트의 문자열은 byte의 컬렉션으로 구현되어 있음

// 8.2.1 문자열이란 무엇일까?
// 러스트는 언어의 명세 안에서 오직 한 가지 문자열 타입, str 타입만을 지원한다.
// 이 타입은 주로 값을 대여한 &str 형태로 보게 될 것이다.
// 문자열 슬라이스는 어딘가에 UTF-8 형식으로 인코딩되어 저장된 문자열에 대한 참조이다.
// 예를 들어, 문자열 리터럴은 프로그램을 컴파일한 바이너리 결과 파일에 포함되므로, 문자열 슬라이스다.

//
// String vs str

// String is the dynamic heap string type, like Vec:
// use it when you need to "own or modify your string data."

// str is an "immutable sequence of UTF-8 bytes of dynamic length"
// "somewhere in memory". Since the size is unknown,
// one can only handle it behind a pointer(string slice).

// 8.2.2 새 문자열 생성하기

pub fn create_string() {
    let mut string = String::new();

    let str_ = "str";
    let string_: String = str_.to_string();
    // 문자열 리터럴의 to_string() 매서드를 직접 호출할 수 있다.
    let string_: String = "str".to_string();

    // 아래와 같이 문자열을 생성하는 것도 가능하다.
    let string_: String = String::from("string");
}

// 8.2.3 문자열 수정하기
pub fn update_string() {
    // push_str과 push 매서드 활용한 문자열 연결
    let mut s1 = String::from("foo");
    let str1 = "bar";
    s1.push_str(str1);
    println!("{}", str1); // str1은 push한 후에도 소유권을 유지

    let mut s2 = String::from("lo");
    s2.push('l');

    // + 연산자나 format! 매크로를 이용한 문자열 연결
    let s1 = String::from("Hello");
    let s2 = String::from(" World!");
    // + 연산자는 내부적으로 add 매서드를 호출
    let s3 = s1 + &s2;
    // add 매서드는 아래와 같이 선언되어 있다.
    // fn add(self, s: &str) -> String {..
    // add 매서드는 매개변수로 &str 타입을 받는데, 예제에서 s2는 &String 타입..
    // 그럼에도 불구하고 컴파일이 되는 이유는, 컴파일러가 &String 인수를 &str 타입으로 자동 변환
    // 러스트는 강제 역참조를 이용해 &s2:&String 을 &s2[..]로 변환한다.
    // 따라서, add 매서드는 매개변수에 대한 소유권을 갖지 않는다.

    // add 매서드는 첫 번째 매개변수인 self의 소유권을 갖는다.
    // self 매개변수에서는 참조 기호(&)를 사용하지 않았기 때문이다.
    // 즉 add 매서드를 호출하는 과정에서 s1의 소유권이 s3에게 이전된다.
    // + 연산자(add 매서드)는 문자열의 복사가 적지만 여러 문자열을 합치기에는 부적합

    // format! 매크로 활용
    // 소유권의 이전이 발생하지 않고 직관적으로 읽을 수 있음
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let s = format!("{}-{}-{}", tic, tac, toe);
}

// 8.2.4 문자열의 인덱스
// 러스트에서는 인덱스를 이용해 String 값의 일부에 접근하려 하면 에러가 발생한다.
pub fn index_string() {
    let s1 = String::from("hello");
    // let h = s1[0]; cannot be indexed by {integer} 에러 발생

    /* (1) 문자열의 내부
    String은 Vec<u8> 타입을 한 번 감싼 타입이다.
    문자열 "안" 유니코드의 스칼라값은 3byte 공간을 차지
    문자열의 바이트에 인덱스로 접근하면 올바른 유니코드 스칼라 값을 가져올 수 없다.
    */
    let len = String::from("Hola").len(); // 4
    let len2 = String::from("안녕하세요").len(); // 15

    /* (2) 바이트와 스칼라값, 그리고 그래핌 클러스터
    러스트 관점에서 볼 때, 문자열은 크게 바이트(bytes), 스칼라값(scalar values)
    그리고 그래핌 클러스터(grapheme clusters, letter) 등 세 가지로 구분된다.
    _
    예를 들어, 한글 '안녕하세요'는 다음과 같이 u8 값들의 벡터에 저장된다.
    [236, 149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184, 236, 154, 148]
    총 15개의 바이트값이 컴퓨터가 최종적으로 '안녕하세요' 문자열을 저장하는 형태이다.
    _
    이 값을 러스트의 char 타입인 유느코드 스칼라값으로 표현하면 다음과 같다.
    ['안', '녕', '하', '세', '요']
    _
    이 벡터에는 다섯 개의 char 값이 저장되어 있다.
    같은 데이터를 그래핌 클러스터로 표현하면 마찬가지로 '안녕하세요'라는 다섯글자를 볼 수 있다.
    ["안", "녕", "하", "세", "요"]
    _
    이처럼 러스트는 컴퓨터가 저장하는 원본 문자열 데이터를 해석하는 여러 가지 방법을 제공한다.
    _
    러스트가 String 타입에서 인덱스 사용을 지원하지 않는 마지막 이유는
    인덱스 처리에 항상 일정한 시간(O(1))이 소요되어야 하지만, String 타입에 대해서만큼은
    일정한 성능을 보장할 수 없기 때문이다.
    */
}

// 8.2.5 문자열 슬라이스 하기
pub fn slice_string() {
    /* 문자열에 대한 인덱싱은 결과는 바이트, 문자 혹은 그래핌 클러스터를 반환한다.
    그래서 어떤 결과물을 받고 싶은지 명확하게 조건을 명시해야 한다.
    */
    let hello = "안녕하세요";
    let s = &hello[0..3]; // 범위를 이용해서 문자열을 bytes로 slice 하는 것은 지양

    /* 8.2.6 문자열을 순회하는 매서드
    개별 유니코드 스칼라값을 조작해야 한다면 가장 좋은 방법은 chars 매서드를 사용하는 방법
    */
    for c in hello.chars() {
        println!("{}", c);
    }
}
