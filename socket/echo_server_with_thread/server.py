import threading
from socket_ import TCPServerSocket
from thread_ import JobThread


def run():
    server_socket = TCPServerSocket()
    server_socket.bind(host="127.0.0.1", port=12345)
    server_socket.listen()

    while True:
        try:
            conn, addr = server_socket.accept()
            job_thread = JobThread(conn, addr)
            thread = threading.Thread(target=job_thread.run)
            thread.start()
        except KeyboardInterrupt:
            server_socket.close()


if __name__ == "__main__":
    run()
