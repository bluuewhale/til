
fn main() {
    println!("Hello, world!");

    // 매개변수를 받는 함수
    let name:String = "Rust".to_string();
    fn_with_arg(name);

    // Statement vs Expression
    cmp_statement_and_expression();
    
    // 값(return)을 반환하는 함수
    let sum = fn_with_return(1, 2);
    println!("x + y = {}", sum);

    // 흐름제어
    fn_with_branches(5);

    // 반복문
    let five_factorial = fn_with_while(5);
    println!("5! = {}", five_factorial)
}

fn fn_with_arg(name:String) { // 매개변수 타입을 명시적으로 선언
    println!("I'm {}", name);
}

fn cmp_statement_and_expression() {
    // Rust의 대입문은 구문(statement) => 값을 return하지 않음
    // 따라서 아래의 구문은 컴파일 에러 발생
    // let x = let y = 6

    // 반면 코드블럭({})은 표현식(expression) => 값을 return할 수 있음
    let x = 5; 
    let y = {
        let x = 3 ;
        x + 1
    };

    println!("x = {}", x);
    println!("y = {}", y);

}


fn fn_with_return(x:i32, y:i32) -> i32 { // return type을 명시
    x + y
}


fn fn_with_branches(x:i32){ // 입력받은 값이 홀수인지 짝수인지 알려주는 함수
    if x % 2 == 0 {
        println!("입력한 값은 짝수 입니다.");
    }
    else {
        println!("입력한 값은 홀수 입니다.");
    }
}

fn fn_with_while(n:u32) -> u32{ // 입력받은 값의 팩토리얼을 반환하는 함수
    let mut result = 1;
    let mut n_iter = n;

    while n_iter > 1 {
        result *= n_iter;
        n_iter -= 1;
    }
    
    result
}