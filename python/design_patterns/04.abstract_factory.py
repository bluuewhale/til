# https://victorydntmd.tistory.com/300
""" 
서로 관련이 있는 객체들을 통째로 묶어서 팩토리 클래스로 만들고, 
이들 팩토리를 조건에 따라 생성하도록 다시 팩토리를 만들어서 객체를 생성하는 패턴 

Factory of Factory(부모 팩토리)를 만드는 방식이라고 생각하면 됨
자식 팩토리에서는 서로 관련있는 객체들을 한번에 생산할 수 있도록 묶어버림

"""

from abc import ABC, abstractmethod


class Factory(ABC):
    @abstractmethod
    def create(self, manufacturer):
        pass


""" KeyBoard """


class KeyboardFactory(Factory):
    def create(self, manufacturer):
        if manufacturer == "Samsung":
            return SamsungKeyboard()
        elif manufacturer == "LG":
            return LGKeyboard()


class KeyBoard:
    def __init__(self):
        self.manufacturer = None

    def press_key(self):
        print("Hello Keyboard")


class SamsungKeyboard:
    def __init__(self):
        self.manufacturer = "Samsung"
        print("Samsung 키보드 생산 완료")


class LGKeyboard:
    def __init__(self):
        self.manufacturer = "LG"
        print("LG 키보드 생산 완료")


""" Mouse """


class MouseFactory(Factory):
    def create(self, manufacturer):
        if manufacturer == "Samsung":
            return SamsungMouse()
        elif manufacturer == "LG":
            return LGMouse()


class Mouse:
    def __init__(self):
        self.manufacturer = None

    def click(self):
        print("Hello Mouse")


class SamsungMouse(Mouse):
    def __init__(self):
        self.manufacturer = "Samsung"
        print("Samsung 마우스 생산 완료")


class LGMouse(Mouse):
    def __init__(self):
        self.manufacturer = "LG"
        print("LG 마우스 생산 완료")


""" Computer (Usual Facotry Design Pattern) """


class Computer:
    def __init__(self, keyboard, mouse):  # DI
        self.keyboard = keyboard
        self.mouse = mouse
        print(f"키보드: {keyboard.manufacturer}, 마우스: {mouse.manufacturer} 컴퓨터 생산 완료")


class ComputerFacotry(Factory):
    def __init__(self):
        self.keyboard_factory = KeyboardFactory()
        self.mouse_factory = MouseFactory()

    def create(self, manufacturer):
        keyboard = self.keyboard_factory.create(manufacturer)
        mouse = self.mouse_factory.create(manufacturer)
        computer = Computer(keyboard, mouse)

        return computer


""" Abstract Factory (추상 팩토리 클래스) """


class FactoryOfCoumputerFactory:
    def create_factory(self, manufacturer):
        if manufacturer == "Samsung":
            return SamsungComputerFactory()
        elif manufacturer == "LG":
            return LGComuputerFactory()


class SamsungComputerFactory(Factory):
    def create(self):
        keyboard = SamsungKeyboard()
        mouse = SamsungMouse()
        return Computer(keyboard, mouse)


class LGComuputerFactory(Factory):
    def create(self):
        keyboard = LGKeyboard()
        mouse = LGMouse()
        return Computer(keyboard, mouse)


if __name__ == "__main__":
    print("Factory Design Pattern Applied")
    computer_factory = ComputerFacotry()
    samsung_pc = computer_factory.create(manufacturer="Samsung")
    lg_pc = computer_factory.create(manufacturer="LG")
    print("=" * 100)

    print("Abstract Factory Design Pattern Applied")
    factory_of_computer_factory = FactoryOfCoumputerFactory()
    samsung_computer_factory = factory_of_computer_factory.create_factory(
        manufacturer="Samsung"
    )
    lg_computer_factory = factory_of_computer_factory.create_factory(manufacturer="LG")

    samsung_pc2 = samsung_computer_factory.create()
    lg_pc2 = lg_computer_factory.create()
