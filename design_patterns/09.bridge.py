# -*- coding: utf-8 -*-
""" 구조(Structural) 패턴

브릿지 패턴
https://lktprogrammer.tistory.com/35

추상 클래스에 인스턴스 형태로 기능 클래스를 삽입
>> 추상 클래스에서 요구되는 기능의 다양성이 높은 경우, 기능의 구현을 전담하는 클래스를 따로 생성하고 해당 기능을 담당하는 클래스의 인스턴스를 외부에서 주입

구현부에서 추상층을 분리하여 각자 독립적으로 변형이 가능하고 확장이 가능하도록 합니다. 
즉 기능과 구현에 대해서 두 개를 별도의 클래스로 구현을 합니다.

- Abstraction : 기능 계층의 최상위 클래스. 구현 부분에 해당하는 클래스를 인스턴스를 가지고 해당 인스턴스를 통해 구현부분의 메서드를 호출합니다.
- RefindAbstraction : 기능 계층에서 새로운 부분을 확장한 클래스
- Implementor : Abstraction의 기능을 구현하기 위한 인터페이스 정의
- ConcreteImplementor : 실제 기능을 구현합니다.
"""

from abc import ABC, abstractmethod


class Animal(ABC):
    def __init__(self, hunt_handler):
        self.hunt_handler = hunt_handler

    def hunt(self):
        self.hunt_handler.find_quarry()
        self.hunt_handler.detect_quarry()
        self.hunt_handler.attack_quarry()


# hunt hanlder


class HuntHandler(ABC):
    @abstractmethod
    def find_quarry(self):
        pass

    @abstractmethod
    def detect_quarry(self):
        pass

    @abstractmethod
    def attack_quarry(self):
        pass


class AiroHuntHandler(HuntHandler):
    def find_quarry(self):
        print("하늘로 날아간다")

    def detect_quarry(self):
        print("공중에서 먹이를 발견")

    def attack_quarry(self):
        print("하강하여 먹이를 사냥한다")


class AquaHuntHandler(HuntHandler):
    def find_quarry(self):
        print("물 속을 탐색한다")

    def detect_quarry(self):
        print("물 속에서 먹이를 발견한다")

    def attack_quarry(self):
        print("먹잇감을 사냥한다")


# animals
class Eagle(Animal):
    def hunt(self):
        print("매의 사냥 방식")
        return super().hunt()


class Shark(Animal):
    def hunt(self):
        print("상어의 사냥 방식")
        return super().hunt()


if __name__ == "__main__":
    airo_hunt_handler = AiroHuntHandler()
    aqua_hunt_handler = AquaHuntHandler()

    eagle = Eagle(hunt_handler=airo_hunt_handler)
    shark = Shark(hunt_handler=aqua_hunt_handler)

    eagle.hunt()
    print("=" * 50)
    shark.hunt()
