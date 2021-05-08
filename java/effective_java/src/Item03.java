import java.lang.reflect.Constructor;
import java.util.function.Supplier;

// 03. Enforce the singleton property with a private constructor or an enum type

// [ Tips ]
// 1. 생성자를 private method로 선언하여, 새로운 객체가 만들어지는 것을 방지한다.
// 2. 실제 객체는 class의 static 멤버필드로 선언하여 하나만 존재하도록 한다.
// 3. serialize 후, deserialize하는 과정에서 인스턴스가 여러 개 생길 수 있으므로 주의 (readResolve implement하여 해결)
public class Item03 {

    public static class Singleton {
        public static final Singleton instance = new Singleton();

        private Singleton() {
        }

        public static Singleton getInstance() {
            return instance;
        }

        private Object readResolve() {
            return instance;
        }
    }

    private static final String Supplier = null;

    public static void main(String[] args) {
        Singleton s1 = Singleton.getInstance();
        Singleton s2 = Singleton.getInstance();
        assert s1.equals(s2);

        // reflection을 사용하여 private 생성자를 호출하면, 예외적으로 2개 이상의 instance 생성 가능
        Singleton s3 = null;
        try {
            Constructor constructor = Singleton.class.getDeclaredConstructor();
            constructor.setAccessible(true);
            s3 = (Singleton) constructor.newInstance();
        } catch (Exception e) {
            e.printStackTrace();
        }
        assert !s1.equals(s3);

        // static 팩토리 메서드를 사용하여 싱글톤 객체를 생성하면,
        // static reference가 필요한 함수형 프로그래밍에서도 사용할 수 있다.
        Supplier<Integer> intSupplier = () -> {
            int num = (int) (Math.random() * 6) + 1; // 주사위 눈금
            return num;
        };
        System.out.println(intSupplier.get());

        Supplier<Singleton> supplier = Singleton::getInstance;
        assert supplier.get().equals(supplier.get());
    }
}
