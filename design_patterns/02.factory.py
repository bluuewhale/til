# https://thdev.net/320

from abc import ABC, abstractmethod

""" Drinks """


class MenuTable:
    HOT_AMERICANO = 0
    ICE_AMERICANO = 1

from abc import ABC, abstractmethod
class Drink(ABC):
    
    @abstractmethod
    def __init__(self):
        self.price = 0
        self.name = None
        self.is_hot = None
    
    def __repr__(self):
        return f"{self.name}: {self.price}원"


class HotAmericano(Drink):
    def __init__(self, price=4000, name="아메리카노"):
        self.price = price
        self.name = name
        self.is_hot = True


class IceAmericano(Drink):
    def __init__(self, price=4500, name="아이스 아메리카노"):
        self.price = price
        self.name = name
        self.is_hot = False


""" Cafe """


class Cafe(ABC):
    @abstractmethod
    def order(self, order_id=None):
        pass


class AbcCafe(Cafe):
    def order(self, order_id):
        if order_id == MenuTable.HOT_AMERICANO:
            return HotAmericano(price=4000)

        elif order_id == MenuTable.ICE_AMERICANO:
            return IceAmericano(price=4500)


class XyzCafe(Cafe):
    def order(self, order_id):
        if order_id == MenuTable.HOT_AMERICANO:
            return HotAmericano(price=2000)

        elif order_id == MenuTable.ICE_AMERICANO:
            print(ValueError("아이스 아메리카노가 매장의 사정으로 판매 중단되었습니다."))
            return None


if __name__ == "__main__":
    abc_cafe = AbcCafe()
    print("ABC Cafe ", abc_cafe.order(0))
    print("ABC Cafe ", abc_cafe.order(1))

    xyz_cafe = XyzCafe()
    print("XYZ Cafe ", xyz_cafe.order(0))
    print("XYZ Cafe ", xyz_cafe.order(1))
