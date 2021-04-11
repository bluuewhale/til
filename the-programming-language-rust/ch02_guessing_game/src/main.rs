use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    let mut rng = rand::thread_rng(); // random number generator
    let secret_no = rng.gen_range(1, 101);

    println!("정답 목록: {}", secret_no);
    println!("숫자를 맞혀봅시다!");
    println!("정답이라고 생각하는 숫자를 입력하세요.");

    loop {
        let mut guess = String::new(); // mut: 가변 변수(mutable)
        io::stdin().read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다.");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("양의 정수를 입력해주세요!");
                continue;
            }

        };
            
        println!("입격한 값: {}", guess);

        match guess.cmp(&secret_no) {
            Ordering::Less => println!("더 큰 숫자를 입력해보세요!"),
            Ordering::Greater => println!("더 작은 숫자를 입력해보세요!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            },
    }
}
}