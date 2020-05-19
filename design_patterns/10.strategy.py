""" 행위(Behavioral) 패턴
https://gmlwjd9405.github.io/2018/07/06/strategy-pattern.html
https://refactoring.guru/design-patterns/strategy

행위를 클래스로 캡슐화해 동적으로 행위를 자유롭게 바꿀 수 있게 해주는 패턴

유사하지만 다른 작업을 수행하는 여러 전략을 클래스 형태로 캡슐화, 
단순히 전략을 만들거나 교체함으로써 행위를 자유롭게 수정할 수 있음 (OCP)

Bridge와 상당히 유사한 구조를 가짐
 - 구체적인 기능에 대한 구현을 외부 클래스에 위임함 (OCP)
   + 기능에 대한 수정-확장이 매우 유연함

"""

from abc import ABC, abstractmethod

# context
class Pocketmon:
    def set_moving_strategy(self, moving_strategy):
        self.moving_strategy = moving_strategy

    def move(self):
        self.moving_strategy.execute()

    def set_attack_strategy(self, attack_strategy):
        self.attack_strategy = attack_strategy

    def attack(self):
        self.attack_strategy.execute()


# strategy
class Strategy(ABC):
    @abstractmethod
    def execute(self):
        pass


# moving strategies


class MovingStrategy(Strategy):
    def execute(self):
        print('Action "Moving" is selected')


class Swim(MovingStrategy):
    def execute(self):
        super().execute()
        print("Swim!")


class Run(MovingStrategy):
    def execute(self):
        super().execute()
        print("Run!")


class Fly(MovingStrategy):
    def execute(self):
        super().execute()
        print("Fly!")


# attack strategies


class AttackStrategy(Strategy):
    def execute(self):
        print('Action "Attack" is selected')


class ThunderBolt(AttackStrategy):
    def execute(self):
        super().execute()
        print("Thunder Bolt!!")


class WaterBomb(AttackStrategy):
    def execute(self):
        super().execute()
        print("Water Bomb!!")


class FireBreath(AttackStrategy):
    def execute(self):
        super().execute()
        print("Fire Breath!!")


if __name__ == "__main__":
    print("Go Lizardmon!")
    lizardmon = Pocketmon()
    lizardmon.set_moving_strategy(Fly())
    lizardmon.set_attack_strategy(FireBreath())

    lizardmon.move()
    lizardmon.attack()
    print("=" * 50)

    print("Go Kobugi!")
    kobugi = Pocketmon()
    kobugi.set_moving_strategy(Swim())
    kobugi.set_attack_strategy(WaterBomb())

    kobugi.move()
    kobugi.attack()
