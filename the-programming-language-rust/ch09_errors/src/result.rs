/* 9.2 Result 타입으로 에러 처리하기
러스트에서는 함수 호출이 실패했을 때, 회복 가능한 형태로 에러를 처리할 수 있도록 Result 열거자를 제공한다.
따라서, 실패할 가능성이 있는 함수는 Result 열거자를 return한다.

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/
use std::fs::File;
use std::io::ErrorKind;

pub fn handle_err_result() {
    let f = match File::open("hello.txt") {
        // Result<std::fs::File, std::io::Error>
        Ok(file) => file,
        Err(error) => {
            panic!("파일 열기 실패: {:?}", error);
        }
    };
}

// 9.2.1 match 표현식으로 여러 종류의 에러 처리하기
pub fn handle_err_result2() {
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            // 표준 라이브러리에 정의된 io::Error 타입은 io::ErrorKind 타입을 리턴하는 kind 메서드를 제공한다.
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("파일 생성에 실패하였습니다. {:?}", e),
            },
            other_error => panic!("파일을 열지 못했습니다. {:?}", other_error),
        },
    };
}

/* 9.2.2 에러 발생 시 패닉을 발생하는 더 빠른 방법: unwrap과 expect
Result<T, E> 타입은 다양한 작업을  처리하기 위한 여러 가지 도움 메서드를 제공한다.
unwrap(): match 표현식과 정확히 같은 동작을 하는 단축(shortcut) 매서드다.
unwrap() -> 만약 Ok 열것값이라면 Ok 열것값에 저장된 값을 return, Err 열것값이라면 panic! 매크로를 호출
expect() -> unwrap()과 유사하지만, 에러 메시지를 지정할 수 있다.
*/
pub fn quick_panic() {
    let f = File::open("hello2.txt").unwrap();
}

/* 9.2.3 에러 전파하기
실패할 가능성이 있는 함수를 호출하여 에러가 발생한 경우,
이를 함수 안에서 처리하지 않고, 호출자가 처리할 수 있도록 지정할 수 있다.
*/
use std::io;
use std::io::Read;

pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = match File::open("hello2.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    return match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    };
    // File::open()이나 f.read_to_string() 호출 과정에서 실패하더라도
    // 즉시 panic! 매크로가 호출되는 것이 아니라,
    // read_username_from_file() 함수를 호출자가 에러를 핸들링하도록 에러를 전파한다.
}

// (1) 러스트는 에러 전파를 쉽게 할 수 있도록 ? 연산자를 제공한다.
// (2) ? 연산자는 Result 타입을 리턴하는 함수에서만 사용할 수 있다.

pub fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello2.txt")?.read_to_string(&mut s)?;

    return Ok(s);
}

use std::fs;
pub fn read_username_from_file_shortcut_enhanced() -> Result<String, io::Error> {
    return Ok(fs::read_to_string("hello2.txt")?);
}
