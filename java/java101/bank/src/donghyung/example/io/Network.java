package donghyung.example.io;

import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.net.ServerSocket;
import java.net.Socket;

public class Network {

    public static void main(String[] args) throws IOException, InterruptedException {
        // one-shot Server
        new Thread(() -> {
            ServerSocket server = null;
            Socket socket = null;

            try {
                server = new ServerSocket(9000);
                while (true) {
                    socket = server.accept();
                    System.out.println("New Client " + socket);
                    DataInputStream dataIn = new DataInputStream(socket.getInputStream());

                    while (true) {
                        String m = dataIn.readUTF();
                        System.out.println(m);
                        break;
                    }
                    break;
                }
                System.out.println("Close Connection with " + socket);
            } catch (IOException e) {
                e.printStackTrace();
            } finally {
                try {
                    if (socket != null)
                        socket.close();
                    if (server != null)
                        server.close();
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }).start();
        Thread.sleep(1000);

        // Client
        Socket socket = new Socket("localhost", 9000);
        System.out.println("Connected to " + socket);
        DataOutputStream dataOut = new DataOutputStream(socket.getOutputStream());
        dataOut.writeUTF("Hello Socket!");
        Thread.sleep(1000);
    }

}