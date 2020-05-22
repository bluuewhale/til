from abc import ABC, abstractmethod
import socket


class ServerSocket(ABC):
    @abstractmethod
    def bind(self, host: str, port: int):
        pass

    @abstractmethod
    def listen(self):
        pass

    @abstractmethod
    def accept(self):
        pass


class TCPServerSocket(ServerSocket):
    def __init__(self):
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

    def bind(self, host, port):
        self.socket.bind((host, port))

    def listen(self):
        self.socket.listen()

    def accept(self):
        conn, addr = self.socket.accept()
        print(f"Connected by {addr}")
        return conn, addr


class ClientSocket(socket.socket):
    """ 필요한 부분은 override """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.host = None
        self.port = None

    def connect(self, host_port):
        host, port = host_port
        super().connect(host_port)
        self.host = host
        self.port = port
