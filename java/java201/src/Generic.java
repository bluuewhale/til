// Generic

// Generic add stability to your code by making more of your bugs detectable at compile time
// 제네릭은 클래스 내부에서 사용할 데이터 타입을 외부에서 지정하는 기법을 의미한다.
// 제레릭은 다양한 타입의 객체들을 다루는 메서드나 컬렉션 클래스에 컴파일 시의 타입체크를 해주는 기능이다.

// [ 장점 ]
// 1. 형변환을 직접 명시할 필요가 없다.
//    Generic을 사용한 코드는 컴파일러가 형변환을 제공함
// 2. 타입 안정성이 보장된다. 
//    컴파일 과정에서 타입 체크가 가능해진다.
// 3. 코드 재사용성이 높아진다.
//    동일한 interface를 가진 객체들에 대해 클래스, 메서드를 공유할 수 있다.

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

import javax.swing.Box;

public class Generic {
    // 제네릭이 없을 때
    public static class SimpleArrayList {
        private int size = 0;
        private Object[] elements;

        public SimpleArrayList(int capacity) {
            this.elements = new Object[Objects.requireNonNull(capacity)];
        }

        public void add(Object value) {
            elements[size++] = value;
        }

        public Object get(int idx) {
            return elements[idx];
        }
    }

    public void testSimpleArrayList() {
        SimpleArrayList list = new SimpleArrayList(5);
        list.add(1);
        list.add("Hi"); // no...!!!

        Integer val1 = (Integer) list.get(0);
        // Integer val2 = (Integer) list.get(1); // this will raise an runtime error;
    }

    // Generic Class
    public static class GenericArrayList<T extends Number> {
        private int size = 0;
        private Object[] elements;

        // DON'T DO THIS
        // private T[] elements;
        // > we cannot create generic array since compiler does not know
        // > if there is enough space for T of which size is not known

        // DON'T DO THIS
        // static T val;
        // > we cannot create a static field using generic

        public GenericArrayList(int capacity) {
            this.elements = new Object[capacity];
        }

        public void add(T value) {
            elements[size++] = value;
        }

        public T get(int idx) {
            return (T) elements[idx];
        }
    }

    public void testGenericArrayList() {
        GenericArrayList<Integer> list = new GenericArrayList<Integer>(5);
        list.add(1);
        list.add("Hi"); // this will raise an compile error
        int val1 = list.get(0);
    }

    // Generic Methods
    // Generic methods often has nothing to do with Generic Class.
    public static class GenericMethodContainer {
        public static <T extends CharSequence> void printFirstChar(T val) {
            // This code works because generic type T implements the same interface as
            // CharSequence;
            System.out.println(val.charAt(0));
        }
    }

    // 와일드카드 서브타이핑(Wildcard and subtyping)
    // Covariant(공변): 구체적인 방향으로 타입 변환을 허용 (자신 + 서브 클래스)
    // Contravariant(반공변): 추상적인 방향으로 타입 변환을 허용 (자신 + 부모 클래스)
    // Invariant (무공변): 오로지 자기 자신의 타입만을 허용한다.
    interface Collection<E> {
        void concat(Collection<E> items);

        void concatFixed(Collection<? extends E> items);
    }

    void testCovariant(Collection<Object> a, Collection<String> b) {
        a.concat(b);
        // Compile Error
        // Collection<Obejct>와 Collectio<String>은 서로 타입 변환이 허용되지 않는 관계이다.

        a.concatFixed(b);
        // This works.
        // Bounded wildcard type(<? extends E>) 을 사용하였다.
        // 메서드 매개변수 items의 inner type은 generic type E와의 공변성(Covariant)을 보장한다.
        // 따라서, 함수의 매개변수로 Obejct의 하위 클래스인 String을 허용한다.
    }

    void testInvariant() {
        List<String> strs = new ArrayList<>();
        List<Object> objs = strs;
        // Compile Error: Cannot convert List<String> to List<Object>
        // String은 Object의 서브타입 이지만, List<String>은 List<Object>의 서브타입이 아니다.
        // 따라서, List<String>은 List<Obejct>으로 타입 변환이 불가능하다.
        // 매개변수화 타입은 무공변(invariant)하다.
    }

    // PESC: Producer-extends, Consumer-super
    // 데이터를 생산하는 곳(Producer)에서는 extends
    // 데이터를 소비하는 곳(Consumer)에서는 super
    static class MyClass {
    }

    public void extendsIsReadable(List<? extends MyClass> list) {
        for (MyClass e : list) { // Ok
            System.out.println(e);
        }
        // 하위 -> 상위 타입으로의 변환은 가능하다.
    }

    public void extendsIsNotWritable(List<? extends MyClass> list) {
        list.add(new MyClass()); // Compile Error
        // 상위 -> 하위 타입으로의 변환은 불가능하다.
    }

    public void superIsNotReadable(List<? super MyClass> list) {
        for (MyClass e : list) { // CompileError
            System.out.println(e);
        }
        // 상위 -> 하위 타입으로의 변환은 불가능하다.
    }

    public void superIsWritable(List<? super MyClass> list) {
        list.add(new MyClass()); // Compile Error
        // 하위 -> 상위 타입으로의 변환은 가능하다.
    }
}
