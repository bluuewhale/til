import threading

from messages import MessageFactory
from validator import MessageValidator
from exceptions import MessageValidateError


class ExchangeServer:
    def __init__(self, conn, addr):
        self.conn = conn
        self.addr = addr

        self.message_factory = MessageFactory()

    def run(self):
        while True:
            try:
                data = self.conn.recv(1024).decode()
            except ConnectionAbortedError:
                continue

            if not data:
                continue

            self.handle_msg(data)

    def handle_msg(self, data: str):
        msg = self.message_factory.create_msg_from_str(data)
        self.conn.sendall(msg)
        self.conn.close()
