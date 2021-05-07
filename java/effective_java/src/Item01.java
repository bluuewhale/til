import Item01.Person.Freshman;

// Item01. Consider static factory methods instead of constructors

// 여기서 말하는 factory methods는 일반적인 design pattern에서 사용되는
// Factory Pattern과는 다른 의미로 사용된다.

// [ Advantages ]
// 1. they have names
//     -> 메서드 명으로, 주어진 파라미터가 어떤 역할을 하는지 이해하기 쉽다.
// 2. they are not required to create a new object each time they are invoked
//     -> immutable class cacheing 
//     -> instance-controlled (singletone, noninstantiable)
// 3. they can return an object of any subtype of their return type.
// 4. the class of the returned object can vary from call to call as a funtion of the input parametes.
// 5. the class of the returned object need not exist when the class containing the method is written.
// .   -> JDBC에서 DB driver에 따라 다른 instance가 return되는데, 이에 대응할 때 유용함

// [ Disadvantages ]
// 1. providing only static factory methods is that classes without public or protected constructors cannot be subclassed
//     -> this can be a blessing in disbuise (composition over inheritance, immutable types)
// 2. factory methods are harder for programmers to find

public class Item01 {
    public static class Person{
        private static final Person JoJo = Person.withNameAge("JoJo", 100); // Advantage 2

        public String name;
        public int age;

        // Advantage 2
        public static Person getJoJo() {
            return JoJo;
        }

        // Advantage 1
        public static Person withName(String name) {
            Person p = new Person();
            p.name = name;
            return p;
        }

        // Advantage 1
        public static Person withAge(int age) {
            Person p = new Person();
            p.age = age;
            return p;
        }

        // Advantage 1
        public static Person withNameAge(String name, int age) {
            Person p = new Person();
            p.name = name;
            p.age = age;
            return p;
        }

        // Advantage 3
        public static Person getFreshman(String name, int age) {
            Freshman f = new Freshman();
            f.name = name;
            f.age = age;
            return f;
        }
        public static class Freshman extends Person {
            String major;

        }

    }

    public static void main(String[] args) {
        // Various static factory methods
        Person peter = Person.withName("peter");
        Person sam = Person.withAge(30);
        assert peter.name == "peter";
        assert sam.age == 30;

        // Get pre-constructed instance
        assert Person.getJoJo().name == "JoJo";

        // Can create subclass of the returned type
        Person.Freshman freshman =  (Person.Freshman) Person.getFreshman("John", 20) ;
        assert freshman instanceof Person.Freshman;
    }
}
