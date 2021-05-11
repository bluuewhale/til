// Item 10: Obey the general contract when overriding equals

// [ 요약 ]
// 논리적 비교가 요구되는 경우를 제외하곤 equals 메서드를 override 하지 말자.
// equals 메서드를 override 하는 경우에는 반드시 지켜야 하는 규칙들이 존재한다.

public class Item10 {

    // Do not override equals when the following conditions apply
    // 1. Each instance of the class is inherently unique
    // -> 상태가 아닌 행동을 정의하는 Thread가 좋은 예시이다.

    // 2. There is no need for the class to provide a "logical equality" test
    // -> equals()는 기본적으로 주소값을 비교한다.
    // -> 만약, 물리적 주소는 달라도 논리적으로 같은 비교 (ex, 문자열 비교)가 필요한 경우는 override해도 괜찮다.

    // 3. A superclass has already overridden equals, and the superclass behavior is
    // apporiate for this class
    // -> AbstractList를 구현한 List처럼 는 상속받은 equals를 그대로 사용해도 되면 override 하지말라.

    // 4. The class is private or package-private, and you are certain that its
    // equals method will never be invoked
    // -> private한 패키지에서 개발자가 확신이 있는 경우를 제외하곤 하지 말라.

    // Equals 메서드를 override해도 되는 경우
    // -> 일반적인 Value Class(value container)에서 logical equality를 제공하기 위해

    public static class TestLogicalEquality {
        public void test() {
            Value x = Value.of(3);
            Value y = Value.of(3);

            assert x.equals(y);
        }

        static class Value {
            private final int inner;

            // 검사 순서
            // 1. 메모리 주소 비교 (동일한 객체인지)
            // 2. 클래스 타입 비교
            // 3. 핵심 필드 비교
            @Override
            public boolean equals(Object o) {
                if (o == this) {
                    return true;
                }
                if (!(o instanceof Value)) {
                    return false;
                }
                Value c = (Value) o;
                if (this.inner == c.inner) {
                    return true;
                }
                return false;
            }

            private Value(int inner) {
                this.inner = inner;
            }

            public static Value of(int inner) {
                return new Value(inner);
            }

        }
    }

    // Equals 메서드를 구현할 때에는 다음의 조건을 만족해야 한다.
    // 1. Reflexive: 자기 자신과의 비교는 항상 true를 반환한다.
    // 2. Symmetric: x.equals(y)는 항상 y.equals(x)와 같다.
    // 3. Transitive: x.equals(y), y.equals(y)가 true이면, x.equals(z)도 true를 반환해야 한다.
    // 4. Consistent: 동일한 x,y에 대한 x.equals(y)는 항상 같은 값을 반환해야 한다.
    // 5. null과의 비교(ex, x.equals(null))는 항상 false를 반환한다.

    public static void main(String[] args) {
        new TestLogicalEquality().test();
    }

}