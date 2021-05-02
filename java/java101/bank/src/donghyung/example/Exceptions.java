package donghyung.example;

public class Exceptions {
    public static void firstMethod() throws Exception {
        secondMethod();
    }

    public static void secondMethod() throws Exception {
        thirdMethod();
    }

    public static void thirdMethod() throws Exception {
        System.out.println("10 / 0 = " + 10 / 0);
    }

    public static void main(String[] args) {
        int i = 10;
        int j = 0;

        try {
            int r = i / j;
        } catch (Exception e) {
            e.printStackTrace();
            System.out.println("Exception: " + e.getMessage());
        } finally {
            System.out.println("Finally");
        }
        System.out.println("Exception successfully handled");

        try {
            firstMethod();
        } catch (Exception e) {
            e.printStackTrace();
            System.out.println("Exception: " + e.getMessage());
        } finally {
            System.out.println("Finally");
        }
        System.out.println("Exception successfully handled");
    }
}