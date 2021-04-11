// 고급 라이프타임
// 모든 참조자는 수명을 갖지만, 거의 대부분의 경우 러스트가 라이프타임을 생략하도록 해준다

// 라이프 타임의 세가지 고급 기능
// 1. 라이프타임 서브타이핑(subtyping): 한 라이프타임이 다른 라이프타임보다 오래 사는 것을 보장
// 2. 라이프타임 바운드: 제네릭 타입을 가르키는 참조자를 위한 라이프타임 명시하기
// 3. 트레잇 객체 라이프타임 추론: 컴파일러가 어떻게 트레잇 객체의 라이프타임을 추론하며, 언제 명시해야 할지

// 라이프타임 서브타이핑은 하나의 라이프타임이 다른 것보다 오래 사는 것을 보장합니다.
// 라이프타임 서브타이핑은 하나의 라이프타임이 다른 라이프타임보다 오래 살아야 함을 명시합니다.

fn test_lifetime_subtyping() {
    struct Context<'s>(&'s str);

    struct Parser<'c, 's: 'c> {
        // 's가 최소한 'c 보다는 오래 산다는 의미의 애노테이션
        context: &'c Context<'c>, // Context 구조체 참조자에 대한 수명을 알 수 없다..
    }

    impl<'c, 's> Parser<'c, 's> {
        fn parse(&self) -> Result<(), &'s str> {
            Err(&self.context.0[1..])
        }
    }

    fn parse_context(context: Context) -> Result<(), &str> {
        //
        // Parser { context: &context }.parse()
        //
        //
        // context 변수의 소유권이 parse_context 함수에게 넘어가고 Parser는 context 변수의 소유권을 가져간다.
        // 그런데, Parser와 Parser의 parse 메서드가 반환한 값은
        //  Context 구조체와 동일한 수명을 가지기 때문에, parse_context 메서드를 벗어나면 모두 죽어버린다.
        //
        // Parse 구조체의 parse 메서드에서 반환된 값은,
        // Parser나 Context 구조체 보다 오래 살아야 한다.
        // 즉, parse_context의 반환 값은 context 구조체의 문자열 슬라이스의 수명과 일치해야 한다.
    }
}

// 제네릭 타입에 대한 참조자 상의 라이프타임 바운드
// T 제네릭 타입은 어떠한 타입도 될 수 있으며, 최소한 'a 라이프 타임보다는 오래 살 수 있다.
struct Ref<'a, T: 'a>(&'a T);
struct StaticRef<T: 'static>(&'static T);

// 트레잇 객체 라이프타임 추론
// 트레잇 객체는 동적 디스패치를 이용할 수 있게 해준다.
trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

// 트레잇 객체의 기본 라이프타임은 'static
// 'a Trait 혹은 'a mut Trait을 쓴 경우, 기본 라이프타임은 'a
// 단일 T:'a 구절을 쓴 경우, 트레잇 객체의 기본 라이프타임은 'a 입니다.

#[test]
fn test_trait_instance_life_time() {
    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<Red>; // 동적 디스패칭
}
