import socket
from socket_ import ClientSocket

if __name__ == "__main__":
    client_socket = ClientSocket(socket.AF_INET, socket.SOCK_STREAM)
    client_socket.connect(("127.0.0.1", 12345))

    # 메시지 전송(echo)
    print("=" * 100)
    client_socket.sendall("Hello Socket!".encode())

    # 메시지 수신
    data = client_socket.recv(1024).decode()
    print(f"Received Data from {client_socket.host}:{client_socket.port}", data)

    # 서버에서 method 실행 요청
    print("=" * 100)
    client_socket.sendall("say_hi".encode())

    # 메시지 수신
    data = client_socket.recv(1024).decode()
    print(f"Received Data from {client_socket.host}:{client_socket.port}", data)

    # 서버 연결 종료 요청
    print("=" * 100)
    client_socket.sendall("close".encode())

    client_socket.close()
