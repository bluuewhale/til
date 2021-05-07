// Item01. Consider static factory methods instead of constructors

// 여기서 말하는 factory methods는 일반적인 design pattern에서 사용되는
// Factory Pattern과는 다른 의미로 사용된다.

// [ Advantages ]
// 1. they have names
// 2. they are not required to create a new object each time they are invoked
//     -> immutable class cacheing 
//     -> instance-controlled (singletone, noninstantiable)
// 3. they can return an object of any subtype of their return type.
// 4. the class of the returned object can vary from call to call as a funtion of the input parametes.
// 5. the class of the returned object need not exist when the class containing the method is written.

// [ Disadvantages ]
// 1. providing only static factory methods is that classes without public or protected constructors cannot be subclassed
//     -> this can be a blessing in disbuise (composition over inheritance, immutable types)
// 2. factory methods are harder for programmers to find

public class Item01 {

    // this method never creates an object (advantage 2)
    public static Boolean valueOf(boolean b) {
        return b? Boolean.TRUE : Boolean.FALSE;
    }

    // Class
    public static class Person{
        String name;
        int age;

        public String getName() {
            return this.name;
        }

        public void setName(String name) {
            this.name = name;
        }

        public int getAge() {
            return this.age;
        }

        public void setAge(int age) {
            this.age = age;
        }

        public static Person withName(String name) {
            Person p = new Person();
            p.name = name;
            return p;
        }

        public static Person withAge(int age) {
            Person p = new Person();
            p.age = age;
            return p;
        }
    }

    public static void main(String[] args) {
        Person peter = Person.withName("peter");
        Person sam = Person.withAge(30);

        assert(peter.getName() == "peter");
        assert(sam.getAge() == 30);
    }
}
