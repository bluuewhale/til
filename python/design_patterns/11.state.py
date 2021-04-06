""" 행동(Behavior) 패턴
https://victorydntmd.tistory.com/294
https://refactoring.guru/design-patterns/state/python/example#lang-features
https://dailyheumsi.tistory.com/212?category=855210

현재 상태(state)에 따라 행동을 다르게 해야하는 경우,
Client가 state를 확인하고 행동을 직접 선택할 필요 없이, 상태 자체를 속성이 아닌 클래스로 정의함으로써 인터페이스를 통일

일반적인 경우, 객체를 추상화할 때, 행동의 주체를 Class, 행동을 method로, 행동 주체의 상태는 property으로 정의된다.
주체의 상태에 따라 행동을 달리해야 하는 경우, property를 기준으로 분기하여 method를 실행하게 된다.
이러한 경우, 상태의 종류가 많아질 수록 분기점(조건문)이 많아지고 코드의 가독성이 떨어지게 된다는 단점이 있다.
이를 보완하기 위해 상태 자체를 객체(property -> class)로 만들고 현 상태에서 취해야 하는 행동을 이 객체에 포함시킨다.
>> 행동의 주체가 되는 클래스가 행동을 가지고 있는 것이 아닌, 상태 클래스가 행동을 갖고 있게 되는 것.

장점
- 조건문이 많은 경우, 이를 대체하고 가독성을 높일 수 있다.
- 상태가 추가되거나 확장되어도, 주체(Context)에서 코드의 수정이 발생하지 않는다(OCP)

단점
- 클래스 수가 많아진다
- 행동과 상태 사이에 강력한 결합이 생겨난다 (장점이자 단점)

Strategy와 유사하지만 차이를 보임

추상화
- Strategy: 행동(method)을 클래스화 시킴
- State: 상태(property)를 클래스화 시킴

목적
- Strategy: 구성을 이용하여 상속을 대체
- State: 구성을 이용하여 코드내의 조건문들을 대체
"""

from abc import ABC, abstractmethod


class Context(ABC):
    _state = None

    def __init__(self, state):
        self.transition_to(state)

    def requestAttack(self):
        return self._state.attack()

    def requestMove(self):
        return self._state.move()

    def requestJump(self):
        return self._state.jump()

    def transition_to(self, state):
        print(f"Context: Transition to {type(state).__name__}")
        self._state = state
        self._state.context = self


class State(ABC):
    """
	The base State class declares methods that all Concrete State should
	implement and also provides a backreference to the Context object,
	associated with the State. This backreference can be used by States to
	transition the Context to another State.
	"""

    @property
    def context(self) -> Context:
        return self._context

    @context.setter
    def context(self, context: Context) -> None:
        self._context = context

    @abstractmethod
    def attack(self):
        pass

    @abstractmethod
    def move(self):
        pass

    @abstractmethod
    def jump(self):
        pass


class OnGroundState(State):
    def attack(self):
        print("Ground Attack!")
        # self.context.transition_to(OnGroundState())

    def move(self):
        print("Moving!")
        # self.context.transition_to(OnGroundState())

    def jump(self):
        print("Jump!")
        self.context.transition_to(JumpState())


class JumpState(State):
    def attack(self):
        print("Jump Attack!")
        self.context.transition_to(OnGroundState())

    def move(self):
        print("Unable to move when jumping")
        self.context.transition_to(OnGroundState())

    def jump(self):
        print("Unable to jump when jumping")
        self.context.transition_to(OnGroundState())


if __name__ == "__main__":
    context = Context(OnGroundState())

    context.requestAttack()
    context.requestMove()

    context.requestJump()
    context.requestAttack()

    context.requestMove()
