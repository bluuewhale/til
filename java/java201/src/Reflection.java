import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.sql.Ref;

/* Reflection(리플렉션)
클래스 로딩이 이미 끝난 클래스에서, 다른 클래스를 동적으로 로딩하여 
로딩된 클래스의 생성자, 필드, 메서드에 접근할 수 있도록 해주는 기능.

런타임에 클래스 구조(ex, 메서드, 필드)를 파악해야 하는 경우 활용된다.
*/
public class Reflection {
    static void reflectVector() {
        try {
            Class vectorClass = Class.forName("java.util.Vector");
            Field[] fields = vectorClass.getDeclaredFields();
            Method[] methods = vectorClass.getDeclaredMethods();

            for (Field f : fields) {
                System.out.println("Field: " + f.getName());
            }

            for (Method m : methods) {
                System.out.println("Method: " + m.getName() + "()");
            }

        } catch (ClassNotFoundException e) {
            e.printStackTrace();
        }
    }

    public static void main(String[] args) {
        Reflection.reflectVector();
    }
}
