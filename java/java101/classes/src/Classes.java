public class Classes {
    public enum Color {
        Black, White, Gray,
    }

    class AutoMobile {
        public String name;
        public Color color;
        public int price;

        public void run() {
        }

        public void printInfo() {
        }
    }

    static class Car {
        public String name;
        public Color color;
        public int price;

        public Car(String name, Color color, int price) {
            this.name = name;
            this.color = color;
            this.price = price;
        }

        @Override
        public void finalize() throws Throwable {
            System.out.println("remove");
            this.printInfo();
        }

        public void run() {
            System.out.println("Brrrrrr");
        }

        public void printInfo() {
            System.out.println("Name: " + name);
            System.out.println("Color: " + color);
            System.out.println("Price: " + price);
        }
    }

    public static void main(String[] args) throws Exception {
        Car sonata = new Car("Sonata", Color.White, 20000000);
        Car grandeur = new Car("Grandeur", Color.Black, 30000000);
        sonata.printInfo();
        sonata.run();
        grandeur.printInfo();
        grandeur.run();
    }
}
