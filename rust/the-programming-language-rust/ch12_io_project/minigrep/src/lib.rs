// 12.1 명령줄 인수 처리하기

// 12.1.1 인수의 값 읽어오기
// 전달된 명령줄 인수를 읽으려면 러스트 표준 라이브러리의 함수(std::env::args)를 사용하자
// 이 함수는 명령줄 인수의 반복자(iterator)를 리턴한다.
use std::env;

pub fn read_args11() -> Vec<String> {
    return std::env::args().collect();
}

pub fn parse_config11(args: &[String]) -> (&str, &str) {
    let query = args[1].as_str();
    let filename = args[2].as_str();
    return (query, filename);
}

// 12.2 파일 읽기
use std::fs;

pub fn read_file() {
    let args = read_args();
    let filename = &args[2];

    let contents = fs::read_to_string(filename)
        .expect(format!("{} 파일을 불러오는데 실패하였습니다.", filename).as_str());

    println!("{}", contents);
}

// 12.3 모듈화와 에러 처리 향상을 위한 리팩토링

// Issue 01. 작업을 쪼개서 함수 마다 하나의 기능을 수행하도록 하자
// Issue 02. 프로그램의 설정 변수를 하나의 구조체에 모아서 그 목적을 명확히 하자
// Issue 03. 파일 열기에 실패했을 때, 에러 처리를 더 명확히 하자
// Issue 04. 모든 에러 처리 코드를 한 곳으로 모으자

// 12.3.1 바이너리 프로젝트의 관심 분리
// 프로그램을 main.rs와 lib.rs 파일로 분리하고 프로그램의 로직을 lib.rs로 옮긴다.
// 명령줄 구문분석 로직이 충분히 작다면 main.rs에 남겨둔다
// 복잡해지면 lib.rs로 옮기자

// main 함수에 남겨질 기능
// 인숫값을 이용해 명령줄 구문분석 로직을 호출
// 기타 다른 설정 적용
// lib.rs 파일의 run 함수 호출
// run 함수가 에러를 리턴할 경우 이에 대한 처리

// main.rs 파일(바이너리 크레이터)는 프로그램의 실행을 담당
// lib.rs 파일(라이브러리 크레이터)는 모든 작업의 로직을 담당
pub fn read_args() -> Vec<String> {
    return std::env::args().collect();
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();
        return Ok(Self {
            query,
            filename,
            case_sensitive,
        });
    }
}

use std::error::Error;

pub fn run<'a>(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };
    //
    for line in result {
        println!("{}", line);
    }
    return Ok(());
}

// 12.4 TDD 방법론으로 개발하기

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line.trim());
        }
    }
    return result;
}
pub fn search_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];

    let query = query.to_lowercase();
    for l in contents.lines() {
        if l.to_lowercase().contains(&query) {
            result.push(l.trim())
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_sensitive_search() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn test_insensitive_search() {
        let query = "rUsT";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.
            Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive(query, contents)
        )
    }
}
