from abc import ABC, abstractmethod
import json


class JsonSerializerMixin:
    """ 클래스의 property를 json 직렬화하여 반환하는 기능을 담은 mixin """

    def to_json(self):
        return json.dumps(self.__dict__)


class Cafe:
    @classmethod
    def say_hi(cls):
        print(f"Welcome to {cls.__name__}")


class CafeABC(Cafe, JsonSerializerMixin):
    def __init__(self):
        self.americano = 4000
        self.latte = "4500"
        super().__init__()


if __name__ == "__main__":
    cafe_abc = CafeABC()

    cafe_abc.say_hi()
    print(type(cafe_abc.to_json()), cafe_abc.to_json())
