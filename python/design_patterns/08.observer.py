""" 옵저버 패턴
https://lee-seul.github.io/concept/design-patterns/2017/03/20/design-pattern-06-observer.html

어떤 객체에 이벤트가 발생했을 때, 관련된 객체(subscriber)에게 통지하도록 하는 디자인 패턴

구성
  - Observer: 특정 객체(Subject)를 관찰하고 정보를 타 객체(Subscriber)에 전달하는 "전달자"
  - Subscriber: Observer로 부터 정보를 전달받는 "수신자"
  - Subject: Observer에 관찰 대상, 정보를 생산하는 "생산자"

예를 들어, 사용자(Subscriber)가 유투브 채널A(Subject)를 구독하면 ObserverA에 등록됨
유투브 채널A에 새로운 동영상이 올라오면 
채널A에서는 ObserverA에 메시지를 전달하고, ObserverA는 구독자(Subscriber)에게 메시지를 전달

구독자(Subscriber)와 채널(Subject)를 직접 연결하지 않는 이유?
>>> Observer를 활용하면 Subscriber와 Subject가 강하게 연결되는 것을 방지
    시스템이 유연해지고 객체간의 의존성도 제거됨 


"""

from abc import abstractmethod, ABC


class Observer:
    def __init__(self):
        self.subscribers = []
        self.msg = ""

    def notify(self):
        for sub in self.subscribers:
            sub.msg = self.msg

    def set_msg(self, msg):
        self.msg = msg

    def register(self, *observer):
        for obsv in observer:
            self.subscribers.append(obsv)

    def unregister(self, *observer):
        for obsv in observer:
            self.subscribers.remove(obsv)


class Subscriber:
    """ 옵저버를 통해 정볼르 전달 받는 객체 """

    def __init__(self):
        self.msg = ""

    def update(self):
        return self.msg


class Subject:
    """ 옵저버에 의해 관찰되는 객체 """

    def __init__(self):
        self.observers = []

    def notify_observer(self, msg):
        for obsv in self.observers:
            obsv.set_msg(msg)
            obsv.notify()

    def attach(self, *observer):
        for obsv in observer:
            self.observers.append(obsv)

    def dettach(self, *observer):
        for obsv in observer:
            self.observers.remove(obsv)


if __name__ == "__main__":
    a = Subscriber()
    b = Subscriber()
    c = Subscriber()

    ob1 = Observer()
    ob2 = Observer()
    ob1.register(a, b)
    ob2.register(c)

    sub = Subject()
    sub.attach(ob1)

    sub.notify_observer("Hello")
    print("A says: ", a.update())
    print("B says: ", b.update())
    print("C says: ", c.update())

    sub.attach(ob2)
    sub.notify_observer("Observer 2 has joined!")
    print("A says: ", a.update())
    print("B says: ", b.update())
    print("C says: ", c.update())
