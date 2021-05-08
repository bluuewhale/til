import java.lang.reflect.Constructor;

// Item04: Enforce noninstantiability with a private constructor

// 일부 클래스들은, 기능적 편의를 위해 여러 클래스와 메서드를 모아두는 용도로 사용된다.
// 이러한 클래스는 instance를 만들기 위해 설계된 것이 아니므로, 원칙적으로 인스턴스 생성을 막아야 한다.
// abstract class로 선언하여도, 이를 상속받는 경우에는 instance 생성을 막을 수 없다.

// -> private constructor를 선언하고 error를 throw하는 방식으로 대처할 수 있다.
public class Item04 {

    // This class should not be instantiated
    public static class MathUtility {
        private static final double pi = 3.141592;
        private static final double exp = 2.71828;
        private static final double goldenRatio = 1.61803;

        protected MathUtility() throws AssertionError {
            throw new AssertionError();
        }
    }

    public static class PhysicsUtility extends MathUtility {
        public PhysicsUtility() {
            super();
        }
    }

    public static void main(String[] args) {
        MathUtility math = new MathUtility();
        PhysicsUtility physics = new PhysicsUtility();
    }
}