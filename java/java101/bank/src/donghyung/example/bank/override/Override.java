package donghyung.example.bank.override;

public class Override {
    public static class Person {
        private String name;

        public Person(String name) {
            this.name = name;
        }

        public String getName() {
            return name;
        }

        public void setName(String name) {
            this.name = name;
        }

        public void printInfo() {
            System.out.println("Name: " + name);
        }
    }

    public static class Employee extends Person {
        private String role;
        private int salary;

        public Employee(String name, String role, int salary) {
            super(name);
            this.role = role;
            this.salary = salary;
        }

        public int getSalary() {
            return salary;
        }

        public void setSalary(int salary) {
            this.salary = salary;
        }

        public String getRole() {
            return role;
        }

        public void setRole(String role) {
            this.role = role;
        }

        // @Override
        public void printInfo() {
            super.printInfo();
            System.out.println("Role: " + role);
            System.out.println("Salary: " + salary);
        }

    }

    public static void main(String[] args) {
        Employee kyle = new Employee("kyle", "Software Engineer", 5000);
        kyle.printInfo();

        Person[] members = new Person[3];
        members[0] = new Person("Jack");
        members[1] = new Employee("John", "HR Manager", 5000);
        for (int i = 0; i < members.length; i++) {
            members[i].printInfo();
        }
    }
}
