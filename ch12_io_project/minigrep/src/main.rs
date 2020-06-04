use ::minigrep as grep;
use std::process;

fn main() {
    let args = grep::read_args();
    let config = grep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        process::exit(1);
    });

    if let Err(e) = grep::run(config) {
        eprintln!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}
