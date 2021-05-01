import java.util.Arrays;

public class Variables {

    public static void main(String[] args) {
        // 원시 자료형
        int in = 10;
        long l = 1111;
        double d = 3.;
        boolean b = true;
        String s = "Hello Java World!";

        System.out.printf("i = %d \n", in);
        System.out.printf("l = %d \n", l);
        System.out.printf("d = %f \n", d);
        System.out.printf("b = %s \n", b);

        System.out.printf("s = %s \n", s);

        // Array
        int[] arr1 = new int[3];
        arr1[0] = 1;
        arr1[1] = 2;
        arr1[2] = 3;
        System.out.println("arr1 = " + Arrays.toString(arr1));
        System.out.println("length of arr1 = " + arr1.length);

        // Conditional Statements
        int num1 = 10;
        int num2 = 20;
        if (num1 > num2) {
            System.out.printf("%s > %s \n", num1, num2);
        } else if (num1 == num2) {
            System.out.printf("%s = %s \n", num1, num2);
        } else {
            System.out.printf("%s < %s \n", num1, num2);
        }

        int score = 60;
        switch (score) {
            case 100:
                System.out.println("수");
            case 90:
                System.out.println("우");
            case 80:
                System.out.println("미");
            case 70:
                System.out.println("양");
            case 60:
                System.out.println("가");
        }

        // Iterations
        for (int i = 1; i < 10; i++) {
            for (int j = 1; j < 10; j++) {
                System.out.printf("%d x %d = %d\n", i, j, i * j);
            }
        }

        int y = 0;
        while (y < 10) {
            System.out.println("y = " + y);
            y += 1;
        }

        do {
            System.out.println("do always run code at least once");
        } while (false);
    }
}