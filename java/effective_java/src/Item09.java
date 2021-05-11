import java.io.BufferedReader;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.FileReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

// Item09: Prefer try-with-resource to try-finally

// Client는 종종 resource closing을 간과하기도 한다.
// 그러므로, 사용 후 반드시 close해야 하는 객체를 사용할 때는 try-with-resource 방식을 사용하라

public class Item09 {

    public static class ExampleTryFinally {

        // 일반적인 try-finally 코드
        public String runSimpleTryFinally(String path) throws IOException {
            BufferedReader br = new BufferedReader(new FileReader(path));

            try {
                return br.readLine();
            } finally {
                br.close();
            }
        }

        // 2개 이상이 되면 굉장히 지저분하다.
        public void runDirtyTryFinally(String src, String dst) throws IOException {
            InputStream in = new FileInputStream(src);
            try {
                OutputStream out = new FileOutputStream(dst);
                try {
                    byte[] buf = new byte[1024];
                    int n;
                    while ((n = in.read(buf)) >= 0) {
                        out.write(buf, 0, n);
                    }
                } finally {
                    out.close();

                }

            } finally {
                in.close();
            }
        }

        // try-with-resource 구문을 사용하면 아주 깔끔하다.
        public void runTryResource(String src, String dst) throws IOException {
            try (InputStream in = new FileInputStream(src); OutputStream out = new FileOutputStream(dst)) {
                byte[] buf = new byte[1024];
                int n;
                while ((n = in.read(buf)) >= 0) {
                    out.write(buf, 0, n);

                }
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
}
