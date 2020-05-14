# https://dailyheumsi.tistory.com/193?category=855210
# https://mino-park7.github.io/python%20study/2018/08/19/python-design-pattern-composite-pattern/
"""
컴퍼지트 패턴은 단일 객체와 그 객체들을 가지는 집합 객체를 같은 타입으로 취급하며, 트리 구조로 객체들을 엮는 패턴이다.

연필통 세트 = 통 + 지우개 + 연필
연필통 선물세트 = 연필통 + 박스 + 포장
"""

from abc import ABC, abstractmethod, abstractproperty
import sys


class AbstractItem(ABC):
    def __iter__(self):
        return iter([])


class Item(AbstractItem):
    def __init__(self, name, price):
        self.name = name
        self.price = price

    def print(self, indent="", file=sys.stdout):
        print("{}${:.2f} {}".format(indent, self.price, self.name))


class AbstractCompositeItem(AbstractItem):
    def __init__(self, *items):
        self.children = []
        if items:
            self.add(*items)

    def add(self, *items):
        self.children.extend(items)

    def remove(self, item):
        self.children.remove(item)

    def __iter__(self):
        return iter(self.children)


class CompositeItem(AbstractCompositeItem):
    def __init__(self, name, *items):
        self.name = name
        super().__init__(*items)

    @property
    def price(self):
        return sum([item.price for item in self])

    def print(self, indent="", file=sys.stdout):
        print("{}${:.2f} {}".format(indent, self.price, self.name))

        for child in self:
            child.print(indent=indent + "\t")


if __name__ == "__main__":
    pencil = Item("pencil", 0.4)
    eraser = Item("eraser", 0.2)
    pencil_case = Item("pencil_case", 1.5)
    pencil_case_set = CompositeItem("pencil_case_set", pencil, eraser, pencil_case)

    box = Item("box", 0.3)
    ribbon = Item("ribbon", 0.2)
    present_set = CompositeItem("present_set", pencil_case_set, box, ribbon)

    for item in (
        pencil,
        eraser,
        pencil_case,
        pencil_case_set,
        box,
        ribbon,
        present_set,
    ):
        item.print()
