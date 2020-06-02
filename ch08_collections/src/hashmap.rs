/* 8.3 키와 값을 저장하는 해시 맵
HashMap<K, V>: K타입의 키에 V 타입의 값을 매핑하여 저장한다
*/

// 8.3.1 새로운 해시 맵 생성하기
// 해시맵은 프렐류드에 등록되어 있지 않으므로, 명시적으로 가져와야 한다.
use std::collections::HashMap;

pub fn create_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("블루팀"), 10);
    scores.insert(String::from("옐로팀"), 50);

    // 튜플의 벡터에 대해 collect() 매서드를 사용해서 해시맵 생성
    // collect() 매서드는 type annotation이 필요함
    let teams = vec![String::from("블루"), String::from("옐로")];
    let score_vec = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(score_vec.iter()).collect();
}

/* 8.3.2 해시 맵과 소유권
i32처럼 Copy 트레이트를 구현하는 타입은 값들이 해시 맵에 복사된다.
반면 String처럼 값을 소유하는 타입은 값이 해시맵으로 이동하며 소유권도 이전된다.
*/

pub fn hashmap_ownership() {
    let filed_name = String::from("Favorite color");
    let field_value = String::from("블루");

    let mut map = HashMap::new();
    map.insert(filed_name, field_value);
    //field_name과 field_value 변수는 이제 유효하지 않다.
}

/* 8.3.3 해시 맵의 값에 접근하기
get() 매서드에 키를 전달하면 된다.
*/

pub fn read_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("옐로"), 50);

    let team_name = String::from("블루");
    let score = scores.get(&team_name);
    // hashmap의 value는 Some(&<_>) 타입으로 저장된다.
    // 해시 맵에 키가 없는 경우 None을 반환해야 하기 때문이다.
    // 따라서, get 매서드를 호출할 때는 Option 타입을 처리해야 한다.

    // for 루프를 이용하면 Key와 Value를 순회할 수 있다.
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

/* 8.3.4 해시 맵 수정하기
각 키에는 오직 하나의 값만 할당할 수 있다.
값 교체 or 덮어쓰기 or 새 값 무시, 조건부 업데이트(키가 없는 경우에만)
*/

pub fn update_hashmap() {
    let mut scores = HashMap::new();

    // 덮어쓰기 (insert)
    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("블루"), 25);

    // 키에 값이 할당되어 있지 않을 때만 추가하기 (entry)
    // entry() -> Entry:enum => Key가 있는지 여부를 반환
    // 이후, Entry 열거자의 or_insert 매서드를 호출하여 값을 추가
    scores.entry(String::from("블루")).or_insert(50); // no update
    scores.entry(String::from("옐로")).or_insert(50); // update

    // 기존 값에 따라 값 수정하기
    let text = "hello world wonderful world";
    let mut word_count_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", word_count_map);
}
