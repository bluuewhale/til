import threading
from sockets import ExchageServerSocket
from exchange import ExchangeServer


def run():
    server_socket = ExchageServerSocket()
    server_socket.bind(host="127.0.0.1", port=12345)
    server_socket.listen()

    while True:
        try:
            conn, addr = server_socket.accept()
            exchange = ExchangeServer(conn, addr)
            th = threading.Thread(target=exchange.run)
            th.start()
        except KeyboardInterrupt:
            server_socket.close()


if __name__ == "__main__":
    run()
