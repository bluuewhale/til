package donghyung.example.bank.employee;

import donghyung.example.bank.bank.Bank;

public class Employee {
    private String name;
    int salary;

    public Employee(String name, int salary) {
        this.name = name;
        this.salary = salary;
    }

    public String getName() {
        return this.name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public void SaveMoney(Bank bank, int money) {
        System.out.println(this.name + "이(가) " + bank.getName() + "에" + money + "원을 저축하였습니다.");
        bank.increaseAccount(money);
    }
}
