package donghyung.example.bank.bank;

public class Bank {
    private String name;
    private static int account = 0;

    public Bank(String name) {
        this.name = name;
    }

    public String getName() {
        return this.name;
    }

    public void increaseAccount(int money) {
        this.account += money;
    }

    public void printInfo() {
        System.out.println(this.name + "의 현재 잔고는 " + this.account + "원 입니다.");
    }
}
