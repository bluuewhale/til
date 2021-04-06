import socket

if __name__ == "__main__":

    HOST = "127.0.0.1"
    PORT = 12345

    # 소켓 객체를 생성
    # 주소 체계(adress family)로 IPv4, 소켓 타입으로 TCP를 사용
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    # 포트 사용중이라는 WinError 10048에러 해결을 위해 필요
    server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)

    # bind() 매서드는 소켓을 특정 네트워크 인터페이스와 포트 번호에 연결하는데 사용됩니다.
    # HOST는 hostname, ip adress, 빈 문자열 ""이 될 수 있습니다.
    # 빈 문자열 ""은 모든 네트워크 인터페이스로부터 접속을 허용합니다.
    # PORT는 1-65535 사이의 숫자를 사용할 수 있습니다.
    server_socket.bind((HOST, PORT))

    # 서버가 클라이언트의 접속을 허용하도록 합니다.
    server_socket.listen()

    # 무한 루프를 돌면서
    while True:
        # accept 함수에서 대기하다가 클라이언트가 접속하면 새로운 소켓을 리턴합니다.
        client_socket, addr = server_socket.accept()

        # 접속한 클라이언트의 주소입니다.
        print("Connected by", addr)

        # 클라이언트가 보낸 메시지를 수신하기 위해 대기합니다.
        data = client_socket.recv(1024)

        # 빈 문자열일 수신하면 루프를 중지합니다.
        if not data:
            break

        print("Received from", addr, data.decode())

        # 받은 문자열을 다시 클라이언트로 전송해줍니다. (에코)
        client_socket.sendall(data)

    client_socket.close()
    server_socket.close()
