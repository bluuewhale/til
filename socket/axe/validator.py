from abc import ABC, abstractmethod


class Validator:
    @abstractmethod
    def validate(self, msg):
        pass


class ClientMessageValidator(Validator):
    MSG_TYPES = [0, 1]

    ORDER_NO_LEN = 5
    TICKER_LEN = 6
    PRICE_LEN = 5
    QTY_LEN = 5

    def validate(self, msg) -> bool:
        for attr in dir(self):
            if attr.startswith("_validate"):
                validate_method = getattr(self, attr)
                validate_method(msg)
        return True

    def _validate_msg_type(self, msg):
        if not isinstance(msg.msg_type, str):
            raise TypeError(f"msg_type should be <str> type")

        if not msg.msg_type in self.MSG_TYPES:
            raise ValueError(f"msg_type should be in {self.MSG_TYPES}")

    def _validate_order_no(self, msg):
        if not isinstance(msg.order_no, str):
            raise TypeError(f"order_no should be <str> type")

        if not len(msg.order_no) == self.ORDER_NO_LEN:
            raise ValueError(f"order_no should be str of length {self.ORDER_NO_LEN}")

    def _validate_ticker(self, msg):
        if not isinstance(msg.ticker, str):
            raise TypeError(f"ticker should be <str> type")

        if not len(msg.ticker) == self.TICKER_LEN:
            raise ValueError(f"ticker should be str of length {self.TICKER_LEN}")

    def _validate_price(self, msg):
        if not isinstance(msg.ticker, str):
            raise TypeError(f"ticker should be <str> type")

        if not len(msg.ticker) == self.PRICE_LEN:
            raise ValueError(f"ticker should be str of length {self.PRICE_LEN}")

    def _validate_qty(self, msg):
        if not isinstance(msg.ticker, str):
            raise TypeError(f"qty should be <str> type")

        if not len(msg.ticker) == self.QTY_LEN:
            raise ValueError(f"qty should be str of length {self.QTY_LEN}")
