import threading

# 스레드 클래스
class JobThread:
    def __init__(self, conn, addr):
        self.conn = conn
        self.addr = addr

    def run(self):
        while True:
            try:
                data = self.conn.recv(1024).decode()
            except ConnectionAbortedError:
                continue

            if not data:
                continue

            method = getattr(self, data, None)
            if method and not method.__name__.startswith("_"):
                result = method()
                if method.__name__ == "close":
                    return

            else:  # echo
                self.conn.send(data.encode())

    # 유저가 quit를 입력했을 경우 종료한다.
    def close(self):
        print(f"Client {self.addr} requested to close")
        self.conn.close()

    def say_hi(self):
        print("Hi")
        self.conn.sendall("Hi".encode())
