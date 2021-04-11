// 사용자 정의 module 호출
mod front_house;

pub use crate::front_house::hosting;

pub fn eat_at_restaurantcar_imported() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// mod 키워드는 모듈을 정의하는 예약어
// 모듈 안에는 다른 모듈을 정의할 수 있다.
// 모듈에는 구조체, 열거자, 상수, 트레이트는 물론 함수도 추가할 수 있다.

// src/main.rs 와 src/lib.rs 파일은 크레이트 루트라고 부른다.
// 그 이유는 두 파일의 콘텐츠가 crate라는 이름의 모듈로 구성되며,
// 이 모듈은 모듈 트리라고 부르는 크레이트의 모듈 구조에서 루트 역할을 하기 때문이다.
mod front_of_house {
    // 고객과 접점이 발생하는 부분
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

// 7.3 경로를 이용해 모듈 트리의 아이템 참조하기
// 절대 경로 vs 상대 경로

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}

// 7.3.2 super로 시작하는 상대 경로
// 상대 경로는 super 키워드를 이용해 부모 모듈로부터 시작할 수도 있다.

// 7.3.3 구조체와 열거자 공개하기
// 구조체를 정의할 때 pub 키워드를 사용한다면 구조체는 공개되지만, 구조체의 필드는 여전히 비공개이다.

fn serve_order() {}

mod back_of_house {
    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    pub struct Breakfast {
        pub toast: String,      // toast 필드는 공개
        seasonal_fruit: String, // seasonal_fruit 필드는 비공개
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    // 여름에 아침 식사로 호밀빵을 주문한다.
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    // 고객이 마음을 바꿔서 빵 종류를 바꾼다.
    meal.toast = String::from("밀빵");
    println!("{} 토스트로 주세요", meal.toast);
}

// 열거자를 공개하면 모든 열것값도 공개된다.alloc
mod back_of_house2 {
    pub enum Appetizer {
        Soup(String),
        Salad(String),
    }
}

pub fn eat_at_restaurant3() {
    let order1 = back_of_house2::Appetizer::Soup(String::from("크렘차우더 스프"));
    let order2 = back_of_house2::Appetizer::Salad(String::from("리코타 치즈 샐러드"));
}

// 7.4 use 키워드로 경로를 범위로 가져오기
// use 키워드로 경로를 현재 범위로 가져오면 경로의 아이템이 마치 현재 범위의 아이템인 것처럼 호출할 수 있다.
// use 키워드는 심볼릭 링크(symbolic link)를 추가하는 것과 유사함 => python의 from A import B
mod front_of_house2 {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use self::front_of_house2::hosting; 상대 경로
// use crate::front_of_house2::hosting; // 절대 경로

pub fn eat_at_restaurant4() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// 7.4.1 관용적인 경로 사용하기
// 함수를 가져올 때는 모듈 이름까지만 가져옴 => 함수가 어디에 정의되어 있는지 명확하게 하기 위해
// 구조체, 열거자 등을 가져올 때는 전체 경로를 다 가져옴 => 코드를 간단하게

// 7.4.2 as 키워드로 새로운 이름 부여하기
// 아래와 같이 새로운 이름을 부여하면 이름 충돌을 피할 수 있다.
use std::fmt::Result;
use std::io::Result as IoResult;

// 7.4.3. pub use 키워드로 re-exporting
mod front_of_house3 {
    pub mod hosting2 {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house3::hosting2;

pub fn eat_at_restaurant5() {
    hosting2::add_to_waitlist();
    hosting2::add_to_waitlist();
}

// 7.4.5 중첩 경로로 use 목록을 깔끔하게 유지하기
use std::io::{self, Write};

// 7.4.6 glob 연산자
// 만일 어떤 경로의 공개 아이템을 모두 가져오려면 글롭 연산자 *를 사용한다.alloc
use std::collections::*;

// 7.5 모듈을 다른 파일로 분리하기
