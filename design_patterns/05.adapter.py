# https://dailyheumsi.tistory.com/189?category=855210
# https://unabated.tistory.com/entry/%EC%96%B4%EB%8C%91%ED%84%B0-%ED%8C%A8%ED%84%B4Adapter-Pattern
"""
1.1. 장점
기존 클라이언트 단의 코드 수정 최소화.
클라이언트는 연동부분을 몰라도, 새로운 코드의 기능을 일관되게 사용가능.

1.3. 활용 상황
기존의 코드에 새로운 코드(써드파티 라이브러리 등)을 연동하여 사용하고 싶은데, 두 코드의 인터페이스가 달라, 이를 하나로 통일하여 사용하고 싶을 때.
아래 예의 경우, 기존의 클라이언트 단 코드에 맞춰 통일함.
 
 - 인터페이스 맞추기
 - 기능 추가
 - 객체 컬렉션 추가
"""

from abc import ABC, abstractmethod


class Cat:
    def meow(self):
        print("meow")


class Dog:
    def bark(self):
        print("bark")


class AnimalAdapter(ABC):
    @abstractmethod
    def make_sound(self):
        pass


class CatAdapter(AnimalAdapter):
    def __init__(self, cat):
        self.cat = cat

    def make_sound(self):
        self.cat.meow()


class DogAdapter(AnimalAdapter):
    def __init__(self, dog):
        self.dog = dog

    def make_sound(self):
        self.dog.bark()


if __name__ == "__main__":
    cat = Cat()
    dog = Dog()

    cat_adapter = CatAdapter(cat)
    dog_adapter = DogAdapter(dog)

    cat_adapter.make_sound()
    dog_adapter.make_sound()
