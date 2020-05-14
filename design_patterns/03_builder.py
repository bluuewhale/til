""" 
https://jdm.kr/blog/217, https://dailyheumsi.tistory.com/187?category=855210

빌더 패턴은 생성자에 들어갈 차례차례 매개 변수를 받아들이고 모든 매개 변수를 받은 뒤에 이 변수들을 통합해서 한번에 사용을 합니다.

장점
    객체 생성에 필요한 파라미터의 의미를 코드 단에서 명확히 알 수 있다. (가독성이 좋다.)
    생성에 필요한 파라미터가 추가될 때 마다, 생성자 오버로딩을 안해도 된다.

활용
    생성자가 많은 경우에 유용
"""


class PersonInfo:
    def __init__(self, name, age, favorite_color, favorite_number, favorite_animal):
        self.name = name
        self.age = age
        self.favorite_color = favorite_color
        self.favorite_number = favorite_number
        self.favorite_animal = favorite_animal

    def __str__(self):
        return f"Name:{self.name}, Age:{self.age}, Favorite Color:{self.favorite_color}, Favorite Number:{self.favorite_number}, Favorite Animal:{self.favorite_animal}"


class PersonInfoBuilder:
    def __init__(self):
        self.name = None
        self.age = None
        self.favorite_color = None
        self.favorite_number = None
        self.favorite_animal = None

    def build(self):
        kwargs = {
            "name": self.name,
            "age": self.age,
            "favorite_color": self.favorite_color,
            "favorite_number": self.favorite_number,
            "favorite_animal": self.favorite_animal,
        }

        return PersonInfo(**kwargs)

    def set_name(self, name):
        self.name = name
        return self

    def set_age(self, age):
        self.age = age
        return self

    def set_favorite_color(self, color):
        self.favorite_color = color
        return self

    def set_favorite_number(self, number):
        self.favorite_number = number
        return self

    def set_favorite_animal(self, animal):
        self.favorite_animal = animal
        return self


if __name__ == "__main__":
    builder = PersonInfoBuilder()

    john = (
        builder.set_name("John Kim")
        .set_age(25)
        .set_favorite_color("Red")
        .set_favorite_number(7)
        .set_favorite_animal("Cat")
        .build()
    )

    assert isinstance(john, PersonInfo)
    print(john)
