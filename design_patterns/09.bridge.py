# -*- coding: utf-8 -*-
""" 구조(Structural) 패턴
https://lktprogrammer.tistory.com/35
https://m.blog.naver.com/PostView.nhn?blogId=tradlinx0522&logNo=220928963011&proxyReferer=https:%2F%2Fwww.google.com%2F


브릿지 패턴
  - 구현 클래스 계층과 기능 클래스 계층을 연결하는 패턴
  - 기능의 구현을 상속이 아닌 위임을 사용(느슨한 연결)

장점
    구현부에서 추상(기능)층을 분리하여 각자 독립적으로 변형이 가능하고 확장이 가능하도록 합니다. 
    즉 기능과 구현에 대해서 두 개를 별도의 클래스로 구현을 합니다.

구성
 + 기능 클래스
  - Abstraction : 최상위 기능 클래스 
  - RefindAbstraction : 하위 기능 클래스 (Abstraction에서 새로운 부분을 "확장"한 클래스)
 + 구현 클래스
  - Implementor : 최상위 구현 클래스  (Abstraction의 기능을 구현하기 위한 인터페이스 정의)
  - ConcreteImplementor : 하위 구현 클래스 (실제 기능을 구현합니다.)
"""

from abc import ABC, abstractmethod


""" Abstraction 
기능 계층의 최상위 클래스. 
구현 부분에 해당하는 클래스를 인스턴스를 가지고 해당 인스턴스를 통해 구현부분의 메서드를 호출합니다.
"""


class Animal:
    def __init__(self, hunt_impl):
        self.hunt_impl = hunt_impl

    def find_quarry(self):
        self.hunt_impl.find_quarry()

    def detect_quarry(self):
        self.hunt_impl.detect_quarry()

    def attack_quarry(self):
        self.hunt_impl.attack_quarry()

    def hunt(self):
        self.find_quarry()
        self.detect_quarry()
        self.attack_quarry()


""" RefindAbstraction 
기능 계층에서 새로운 부분을 확장한 추상 클래스 (구현x 확장!)
"""


class SoundAnimal(ABC, Animal):
    @abstractmethod
    def make_sound(self):
        pass


"""Implementor 
Abstraction의 기능을 구현하기 위한 인터페이스 정의
"""


class HuntImpl(ABC):
    @abstractmethod
    def find_quarry(self):
        pass

    @abstractmethod
    def detect_quarry(self):
        pass

    @abstractmethod
    def attack_quarry(self):
        pass


"""ConcreteImplementor 
실제 기능을 구현합니다. """


class AiroHuntImpl(HuntImpl):
    def find_quarry(self):
        print("하늘로 날아간다")

    def detect_quarry(self):
        print("공중에서 먹이를 발견")

    def attack_quarry(self):
        print("하강하여 먹이를 사냥한다")


class AquaHuntImpl(HuntImpl):
    def find_quarry(self):
        print("물 속을 탐색한다")

    def detect_quarry(self):
        print("물 속에서 먹이를 발견한다")

    def attack_quarry(self):
        print("먹잇감을 사냥한다")


# animals


class Shark(Animal):
    def hunt(self):
        print("상어의 사냥 방식")
        return super().hunt()


class Eagle(SoundAnimal):
    def hunt(self):
        print("매의 사냥 방식")
        return super().hunt()

    def make_sound(self):
        print("scream!!")


if __name__ == "__main__":
    aqua_hunt_impl = AquaHuntImpl()
    airo_hunt_impl = AiroHuntImpl()

    shark = Shark(hunt_impl=aqua_hunt_impl)
    eagle = Eagle(hunt_impl=airo_hunt_impl)

    shark.hunt()
    print("=" * 50)
    eagle.hunt()
    eagle.make_sound()
