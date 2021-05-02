package donghyung.example.bank.interface_;

public abstract class Animal {
    public abstract void makeSound();

    public static class Dog extends Animal {
        @Override
        public void makeSound() {
            System.out.println("Bark!");
        }
    }

    public static class Cat extends Animal {
        @Override
        public void makeSound() {
            System.out.println("Meow!");
        }

    }

    public static void main(String[] args) {
        Animal dog = new Dog();
        Animal cat = new Cat();

        dog.makeSound();
        cat.makeSound();
    }
}
