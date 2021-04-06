from abc import ABC, abstractmethod
import json


class Message(ABC):
    @abstractmethod
    def encode(self):
        pass

    def to_json(self):  # json 포맷을 사용하여 로깅 및 메타 데이터 관리 개선
        return json.dumps(self.__dict__)


""" Client Side Message """


class ClientMessage(Message):
    def __init__(
        self,
        msg_type: str,
        order_no: str,
        ticker: str,
        price: str,
        qty: str,
        *args,
        **kwargs
    ):
        self.msg_type = msg_type
        self.order_no = order_no
        self.ticker = ticker
        self.price = price
        self.qty = qty
        super().__init__(*args, **kwargs)

    def encode(self):
        return "".join(
            [self.msg_type, self.order_no, self.ticker, self.price, self.qty]
        ).encode("ascii")


class NewOrderMessage(ClientMessage):
    def __init__(self, msg_type, order_no, ticker, price, qty, *args, **kwargs):
        super().__init__(msg_type, order_no, ticker, price, qty, *args, **kwargs)


class CancelOrderMessage(Message):
    def __init__(self, msg_type, order_no, ticker, price, qty, *args, **kwargs):
        super().__init__(msg_type, order_no, ticker, price, qty, *args, **kwargs)


""" Server Side Message """


class ServerMessage(Message):
    def __init__(self, msg_type, order_no, is_success, *args, **kwargs):
        self.msg_type = msg_type
        self.order_no = order_no
        self.is_success = is_success
        super().__init__(*args, **kwargs)

    def encode(self):
        return "".join([self.msg_type, self.order_no, self.is_success]).encode("ascii")


class OrderReceivedMessage(ServerMessage):
    def __init__(self, msg_type, order_no, is_success, *args, **kwargs):
        super().__init__(msg_type, order_no, is_success, *args, **kwargs)


""" factory """


class MessageFactory:
    def create(self, msg_type: str, *args, **kwargs):
        if msg_type == "0":
            return NewOrderMessage(msg_type, *args, **kwargs)
        elif msg_type == "1":
            return CancelOrderMessage(msg_type, *args, **kwargs)
        elif msg_type == "2":
            return OrderReceivedMessage(msg_type, *args, **kwargs)

    def create_msg_from_str(self, str_):
        args = (str_[:1], str_[1:6], str_[6:12], str_[12:17], str_[17:22])
        return self.create(*args)

    def create_msg_from_bytes(self, bytes_):
        str_ = bytes_.decode()
        return self.create_msg_from_str(str_)
