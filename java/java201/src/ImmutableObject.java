import java.util.Arrays;
import java.util.Objects;
import java.util.Collection;
import java.util.Collections;
import java.util.List;

public class ImmutableObject {
    // 불변객체(ImmutableObject)란 객체가 생성된 시점 이후로는 내부 상태가 변하지 않는 객체이다.

    // [ 장점 ]
    // 1. 상태가 변하지 않기 떄문에, ThreadSafe하여 multi-threading 환경에서 유용하다.
    // 2. 내부 상태 변화로 인해 발생하는 부수 효과(Side effect) 관련 오류를 막을 수 있다.
    // 3. 상태가 항상 일정하기 때문에, 안전하게 사용할 수 있다.
    // 4. GC 성능을 개선할 수 있다. ()

    // [ 단점 ]
    // 1. 상태 변환을 야기하는 메서드는 일반적으로 방어적 복사(defensive-copy)를 통해 새로운 객체를 생성해서 반환한다.
    // 이 경우, 잦은 copy로 인해 성능 저하가 발생할 수도 있다.

    // [ 구현 방법 ]
    // 1. 내부 필드는 모두 private 혹은 final로 선언한다.
    // 2. private constructor를 구현하여 생성자 호출로 인한 객체 생성을 막는다.
    // 객체 생성은 static factory method에 의해서만 가능하도록 한다.
    // 3. Setter는 구현하지 않는다.
    // 4. Getter로 내부 필드를 참조하여 상태 변화가 발생할 가능성이 있는 경우, (ex, List<?>)
    // 방어적 복사를 통해 새로운 객체를 반환한다.

    public static final class Person {
        // 1. 내부 필드는 모두 private 혹은 final로 선언한다.
        private final String name;
        private final int age;
        private final List<String> hobbies;

        // 2. private constructor를 구현하여 생성자 호출로 인한 객체 생성을 막는다.
        private Person(String name, int age, List<String> hobbies) {
            this.name = Objects.requireNonNull(name);
            this.age = Objects.requireNonNull(age);
            this.hobbies = Objects.requireNonNull(hobbies);
        }

        // 3. Setter는 구현하지 않는다.
        public static Person valueOf(String name, int age, List<String> hobbies) {
            return new Person(name, age, hobbies);
        }

        public int getAge() {
            return this.age;
        }

        public String getName() {
            return this.name;
        }

        // 4. Getter로 내부 필드를 참조하여 상태 변화가 발생할 가능성이 있는 경우, (ex, List<?>)
        // 방어적 복사를 통해 새로운 객체를 반환한다.
        public List<String> getHobbies() {

            return Collections.unmodifiableList(this.hobbies);
        }
    }

    public static void main(String[] args) {
        String name = "Tom";
        int age = 20;
        List<String> hobbies = Arrays.asList("Soccer", "Watching Movie");
        Person tom = Person.valueOf(name, age, hobbies);

        // Immutablity Check
        String name_ = tom.getName();
        name_ = "Sam";
        assert tom.getName() == "Tom";

        // Immutablity Check
        int age_ = tom.getAge();
        age_ = 30;
        assert tom.getAge() == 20;

        // Immutablity Check
        List<String> hobbies_ = tom.getHobbies();
        // hobbies_.add("Listening to Music"); this line will raise an compile error
        assert tom.getHobbies() == Arrays.asList("Soccer", "Watching Movie");
    }
}
