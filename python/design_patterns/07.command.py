"""
https://victorydntmd.tistory.com/295
https://anywayjhwa.tistory.com/11

커맨드 패턴( Command Pattern )
※ Command is a behavioral design pattern that turns a request into a stand-alone object that contains all information about the request. 

커맨드 패턴은 객체의 행위( 메서드 )를 클래스로 만들어 캡슐화 하는 패턴입니다.
커맨드 패턴은 작업 요청 - 작업 처리를 분리시키는 데 아주 좋은 패턴입니다.


즉, 어떤 객체(A)에서 다른 객체(B)의 메서드를 실행하려면 그 객체(B)를 참조하고 있어야 하는 의존성이 발생합니다.

1. 커맨드 패턴을 적용하면 의존성을 제거할 수 있습니다.
2. 기능이 수정되거나 변경이 일어날 때 A 클래스 코드를 수정없이 기능에 대한 클래스를 정의하면 되므로 시스템이 확장성이 있으면서 유연해집니다.


1) 커맨드 패턴에서 알아야 할 용어

커맨드 패턴에서는 총 네가지 용어를 사용합니다.

① 수신자 (Receiver) : 행동을 하는 객체 : 제어 장치 (여러 할 수 있는 행동이 담겨 있을거임) -> 장치

② 커맨드 (Command) : 수신자(Receiver)의 정보 + 행동이 들어 있는 객체 -> 버튼의 행위

③ 발동자 (Invoker) : 커맨드를 저장함 -> 버튼이 어떠한 행동을 할지 설정해 주는 행위

④ 클라이언트 (Client) : 커맨드 객체를 생성, 발동자(Invoker)를 통해 수신자(Receiver)에게 할 행동을 전달함 -> 리모컨


"""

from abc import ABC, abstractmethod


""" Receiver """


class TurnableReceiver:
    on_flag = False

    def on(self):
        setattr(self, "on_flag", True)

    def off(self):
        setattr(self, "on_flag", False)


class Light(TurnableReceiver):
    def __init__(self, loc):
        self.loc = loc
        super().__init__()

    def on(self) -> None:
        print(f"{self.loc} Light On")
        super().on()

    def off(self) -> None:
        print(f"{self.loc} Light Off")
        super().off()


class TV(TurnableReceiver):
    def on(self) -> None:
        print("TV On")
        super().on()

    def off(self) -> None:
        print("TV Off")
        super().off()


# Command
class Command(ABC):
    @abstractmethod
    def execute(self):
        pass

    @abstractmethod
    def undo(self):
        pass


class LightPowerCommand(Command):
    def __init__(self, light: Light):
        self.light = light
        super().__init__()

    def execute(self) -> None:
        state = getattr(self.light, "on_flag")
        if state:
            self.light.off()
        else:
            self.light.on()

    def undo(self) -> None:
        pass


class TVPowerCommand(Command):
    def __init__(self, tv: TV):
        self.tv = tv
        super().__init__()

    def execute(self) -> None:
        state = getattr(self.tv, "on_flag")
        if state:
            self.tv.off()
        else:
            self.tv.on()

    def undo(self) -> None:
        pass


# controler
class Control:
    def __init__(self, cmd: Command):
        self.cmd = cmd

    def execute(self):
        self.cmd.execute()


if __name__ == "__main__":
    # receiver
    living_room_light = Light(loc="living room")
    bathroom_light = Light(loc="bathroom")
    tv = TV()

    # command
    living_room_light_cmd = LightPowerCommand(living_room_light)
    bathroom_light_cmd = LightPowerCommand(bathroom_light)
    tv_cmd = TVPowerCommand(tv)

    # control
    living_room_light_ctl = Control(living_room_light_cmd)
    bathroom_light_ctl = Control(bathroom_light_cmd)
    tv_ctl = Control(tv_cmd)

    living_room_light_ctl.execute()
    living_room_light_ctl.execute()
    bathroom_light_ctl.execute()
    bathroom_light_ctl.execute()
    bathroom_light_ctl.execute()
    tv_ctl.execute()
