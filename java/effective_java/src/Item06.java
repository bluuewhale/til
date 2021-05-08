import java.util.regex.Pattern;

// Item06. Avoid creating unnecessary objects
public class Item06 {

    // Wrapper class를 생성할 때는 주의한다.
    // prefer primivites to boxed primitives
    // and watch out for unintentinal autoboxing!;
    void createString() {
        String s = new String("bin"); // DON'T DO THIS!
        String s2 = "bin"; // DO THIS
    }

    static long runSlowLong() {
        Long sum = 0L; // DON'T DO THIS!
        long start = System.nanoTime();
        for (long i = 0; i <= Integer.MAX_VALUE; i++) {
            sum += i;
        }
        long end = System.nanoTime();
        return end - start;
    }

    static long runFastLong() {
        long sum = 0; // DO THIS!
        long start = System.nanoTime();
        for (long i = 0; i <= Integer.MAX_VALUE; i++) {
            sum += i;
        }
        long end = System.nanoTime();
        return end - start;
    }

    // Boolean.valuOf는 불필요한 객체 생성을 막는 static factory method의 좋은 예시이다.
    // 내부적으로 final 필드로 TRUE와 FALSE를 갖고, 이 중 하나를 반환한다.
    static void runBoolean() {
        Boolean bool = Boolean.valueOf(true);
    }

    // 생성할 때 많은 리소스가 요구되는 객체(ex, Pattern, DB Connection)는 내부적으로 캐싱한다.
    public static class RomanNumeralChecker {
        private final static Pattern ROMAN = Pattern
                .compile("(?=.)M*(C[MD]|D?C{0,3})(X[CL]|L?X{0,3})(I[XV]|V?I{0,3})$");

        public static boolean isRomanNumeral(String s) {
            return ROMAN.matcher(s).matches();
        }
    }

    // 방어적 복사 vs 객체 공유
    // 객체 공유가 방어적 복사보다 메모리/연산 효율적이다.
    // 그러나, 방어적 복사를 해야할 때, 객체 공유를 하면 심각한 버그가 발생할 수 있다는 사실을 인지해야 한다.
    public static void main(String[] args) {
        System.out.println(runFastLong());
        System.out.println(runSlowLong());
        assert runSlowLong() > runFastLong();
    }

}