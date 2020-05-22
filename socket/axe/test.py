import socket

from messages import MessageFactory
from sockets import ClientSocket


def test_message():
    message_factory = MessageFactory()
    kwargs = {
        "msg_type": "0",
        "order_no": "".zfill(5),
        "ticker": "000660",
        "price": "15000",
        "qty": "30".zfill(5),
    }
    msg = message_factory.create(**kwargs)
    bytes_ = msg.encode()
    print(bytes_)

    msg_restored = message_factory.create_msg_from_bytes(bytes_)
    print(msg_restored)


def test_socket_server():
    message_factory = MessageFactory()
    kwargs = {
        "msg_type": "0",
        "order_no": "".zfill(5),
        "ticker": "000660",
        "price": "15000",
        "qty": "30".zfill(5),
    }
    msg = message_factory.create(**kwargs)
    bytes_ = msg.encode()

    client_socket = ClientSocket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket.connect(("127.0.0.1", 12345))
    client_socket.sendall(bytes_)


if __name__ == "__main__":
    test_socket_server()
