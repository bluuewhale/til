import jdk.jshell.spi.ExecutionControl.ExecutionControlException;

// Item08: Avoid finalizers and cleaners

// 대부분의 경우, finalizer와 cleaner는 위험하고, 그렇게 쓸모있지 않다.
// GC의 존재로 인해, Finalizer는 예측 불가능하다. (GC가 언제 작동할지 모른다.)
// System.gc()을 호출하여도 즉시 GC가 동작하는 것은 아니다. (GC 실행을 앞당겨달라는 요청일 뿐)
// 따라서, time-critical한 작업은 절대로 finalizer를 통해 처리하지 말아야 한다.

// Lock을 반환하는 작업을 finalizer를 통해 처리하면, 분산 시스템 자체를 무너트릴 수도 있다.

public class Item08 {

    // 자원 반납은 AutoCloseable 인터페이스를 구현하거나, close()를 명시적으로 호출하라
    public static class AutoCloseResourse implements AutoCloseable {
        private boolean isClosed;

        @Override
        public void close() throws Exception {
            if (this.isClosed) {
                throw new IllegalStateException();
            }
            System.out.println("CLOSE");
            this.isClosed = true;
        }

        // 예외적으로, 안전망(safety net) 차원에서 Stream 계열 클래스들에 finalizer를 구현하기도 한다.
        @Override
        protected void finalize() throws Exception {
            if (!this.isClosed) {
                close();
            }
        }
    }
}
