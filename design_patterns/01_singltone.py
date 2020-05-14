class Singletone1(object):
    """ 생성자 호출 방식 """

    _instance = None

    def __new__(cls, *args, **kwargs):
        if not isinstance(cls._instance, cls):
            print("creating new class instance")
            cls._instance = object.__new__(cls, *args, **kwargs)
        else:
            print("class instance is already created")
        return cls._instance


class MyClass1(Singletone1):
    pass


class Singletone2(object):
    __instacne = None

    @classmethod
    def __get_instance(cls):
        return cls.__instance

    @classmethod
    def instance(cls, *args, **kwargs):
        cls.instance = cls.__get_instance  # instance() cls 매서드를 다시 호출하지 않도록 override
        cls.__instance = cls(*args, **kwargs)  # class 생성자 호출
        return cls.__instance


if __name__ == "__main__":
    class1 = MyClass1()
    class2 = MyClass1()
    class3 = MyClass1()

    print(id(class1), id(class2), id(class3))
    assert id(class1) == id(class2)

    class11 = Singletone2.instance()
    class12 = Singletone2.instance()
    print(id(class11), id(class12))
    assert id(class11) == id(class12)
