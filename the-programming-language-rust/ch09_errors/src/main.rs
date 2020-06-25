/* CH09 에러 처리

러스트는 에러를 크게 회복 가능한(recoverable) 에러와 회복이 불가능한(unrecoverable) 에러 등 두 가지로 구분한다.

러스트에는 에러라는 개념이 없다.
대신 회복 가능한 에러를 표현하는 Result<T, E> 타입과,
회복이 불가능한 에러가 발생한 프로그램을 종료하는 panic! 매크로를 지원한다
*/

use ::errors::example;
use ::errors::panic;
use ::errors::result;
fn main() {
    // panic::run_panic();
    // panic::panic_occurs();
    // result::handle_err_result();
    // result::handle_err_result2();
    // result::quick_panic();
    example::unwrap_recommended();
}
