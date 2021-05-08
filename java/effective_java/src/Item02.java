import java.util.EnumSet;
import java.util.Objects;

// Item02: Consider a builder when faced with many constructor parameters
//   -> 생성자의 매개변수가 많아지면 builder pattern을 적극 활용하자.

// [ 장점 ]
// 읽기 편하다
// 일부 필드는 optional하거나, lazy하게 추가되는 경우에도 유연하게 대응할 수 있다.

// [ Tips ]
// 1. 추상 Builder class에서 build() 메서드가 Covariant한 return type을 반환하면 클라이언트 코드에서 타입 캐스팅을 할 필요가 없어진다.
// 2. 메서드 체인닝을 위해 Builder class에 self() 메서드를 구현하여 사용하면 self-type 개념을 활용할 수 있다.

public class Item02 {

    // bad
    // 생성자에 모든 매개변수를 추가하는 방식
    // 장황하고 각각의 매개변수가 무슨 기능을 하는지 알기 어렵다.
    public static class NutritionFacts01 {
        private int servingSize;
        private int servings;
        private int calories;
        private int fat;
        private int sodium;
        private int carbohydrate;

        public NutritionFacts01(int servingSize, int servings, int calories, int fat, int sodium, int carbohydrate) {
            this.servingSize = servingSize;
            this.servings = servings;
            this.calories = calories;
            this.fat = fat;
            this.sodium = sodium;
            this.carbohydrate = carbohydrate;
        }
    }

    // still bad
    // setter method를 활용하는 방식
    // 코드가 장황해지고, 불완전한 상태로 사용될 여지가 있다.
    // setter, getter로 인해 불변 클래스로 만들지 못한다.
    // setter로 인해, ThreadSafe하지 않으므로, lock이 필요해진다.
    public static class NutritionFacts02 {
        private int servingSize;
        private int servings;
        private int calories;
        private int fat;
        private int sodium;
        private int carbohydrate;

        public NutritionFacts02() {
        }

        public void setServingSize(int servingSize) {
            this.servingSize = servingSize;
        }

        public void setServings(int servings) {
            this.servings = servings;
        }

        public void setCalories(int calories) {
            this.calories = calories;
        }

        public void setFat(int fat) {
            this.fat = fat;
        }

        public void setSodium(int sodium) {
            this.sodium = sodium;
        }

        public void setCarbohydrate(int carbohydrate) {
            this.carbohydrate = carbohydrate;
        }
    }

    // Good
    // 빌더를 사용한 패턴
    public static abstract class Pizza {
        public static enum Topping {
            HAM, MUSHROOM, ONION
        }

        public static enum Size {
            SMALL, MEDIUM, LARGE
        }

        private final EnumSet<Topping> toppings;
        private final Size size;

        // 자기 자신의 하위타입을 받는 빌더, 즉 재귀적인 타입 매개변수
        abstract static class Builder<T extends Builder<T>> {
            private EnumSet<Topping> toppings = EnumSet.noneOf(Topping.class);
            private Size size = Size.SMALL;

            public T addTopping(Topping topping) {
                toppings.add(Objects.requireNonNull(topping));
                return self();
            }

            public T withSize(Size size) {
                size = Objects.requireNonNull(size);
                return self();
            }

            // Return Covariant type (subclasses only)
            abstract Pizza build();

            // `self-type`을 반환함으로써, 메서드 체이닝을 가능하게 한다.
            protected abstract T self();
        }

        private Pizza(Builder<?> builder) {
            toppings = builder.toppings;
            size = builder.size;
        }
    }

    public static class CheesePizza extends Pizza {
        public static enum Cheese {
            Mozzarella, Cheddar
        }

        private final Cheese cheese;

        public static class Builder extends Pizza.Builder<Builder> {
            private Cheese cheese;

            public Builder(Cheese cheese) {
                this.cheese = cheese;
            }

            @Override
            public CheesePizza build() {
                return new CheesePizza(this);
            }

            @Override
            protected CheesePizza.Builder self() {
                return this;
            }

        }

        private CheesePizza(Builder builder) {
            super(builder);
            this.cheese = builder.cheese;
        }
    }

    public static void main(String[] args) {
        CheesePizza CheesePizza = new CheesePizza.Builder(Item02.CheesePizza.Cheese.Mozzarella)
                .addTopping(Pizza.Topping.HAM)
                .addTopping(Pizza.Topping.ONION)
                .withSize(Pizza.Size.LARGE)
                .build();
    }
}
