""" 
프록시(Proxy) 패턴
https://lee-seul.github.io/concept/design-patterns/2017/03/20/design-pattern-05-proxy.html

활용
  - 자주 사용하는 객체를 "캐싱"함으로써 애플리케이션 성능을 향상시킬 수 있다 
  - "로깅" 클래스 구현에 활용
  - 프록시 객체에서 "접근 권한 통제"를 통해 보안 수준을 높일 수 있다.
  - 사용하는 데에 리소스가 많이 필요한 객체의 경우, 새로운 객체를 생성하는 것이 아니라 기존에 생성 해놓은 객체를 사용하고 새로운 참조를 만들어 사용한다.
  
Proxy 객체는 실제 사용할 객체인 RealSubject객체와 동일 인터페이스를 상속받고 Client는 이 Proxy 객체에 RealSubject에 대한 호출을 위임한다.

구조
  - Subject: 기능 클래스(Interface)
  - RealSubject: 구현 클래스 (Client는 접근 불가)
  - Proxy: 구현 클래스와 동일한 인터페이스를 상속 (Client가 접근 가능, 호출 위임)

"""

from abc import ABC, abstractmethod
import inspect


class Subject(ABC):
    @abstractmethod
    def add(self, a: int, b: int) -> int:
        pass

    @abstractmethod
    def subtract(self, a: int, b: int) -> int:
        pass


class RealSubject(Subject):
    def add(self, a, b):
        return a + b

    def subtract(self, a, b):
        return a - b


class RealSubjectProxy(Subject):
    def __init__(self, subject):
        self.subject = subject
        self.n_called = 0

    def add(self, a, b):
        print("add() is being called")
        self.n_called += 1
        return self.subject.add(a, b)

    def subtract(self, a, b):
        print("subtract() is being called")
        self.n_called += 1
        return self.subject.subtract(a, b)


if __name__ == "__main__":
    real_subject = RealSubject()
    proxy1 = RealSubjectProxy(real_subject)
    proxy2 = RealSubjectProxy(real_subject)

    print(proxy1.add(1, 1))
    print(proxy1.subtract(10, 1))
    print(proxy2.add(100, 5))

    print("=" * 50)
    print(proxy1.n_called)
    print(proxy2.n_called)
