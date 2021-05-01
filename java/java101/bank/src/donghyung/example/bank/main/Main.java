package donghyung.example.bank.main;

import donghyung.example.bank.bank.Bank;
import donghyung.example.bank.employee.Employee;

public class Main {

    public static void main(String[] args) {
        Employee brian = new Employee("Brian", 3000);
        Employee kyle = new Employee("kyle", 5000);

        Bank bank = new Bank("Future Bank");
        brian.SaveMoney(bank, 1000);
        kyle.SaveMoney(bank, 3000);

        bank.printInfo();
    }
}
