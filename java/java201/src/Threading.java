public class Threading {

    // Thread 클래스를 상속하는 방식
    public static class PrintThread extends Thread {
        @Override
        public void run() {
            super.run();
            System.out.println("Hello, PrintThread");

        }
    }

    public static class TestPrintThread {
        public void test() {
            new PrintThread().start();
        }
    }

    // Runnable interface를 구현하는 방식
    // 다중상속 제한으로 인해, Thread를 상속하는 방법보다 이 방식이 선호된다.
    // Interface를 구현하는 방식이 더 유연한 API를 제공할 수 있다.
    public static class PrintRunnable implements Runnable {

        @Override
        public void run() {
            System.out.println("Hello, Runnable!");
        }
    }

    public static class TestPrintRunnable {
        public void test() {
            new Thread(new PrintRunnable()).start();
            new Thread(() -> System.out.println("Hello, Lambda Thread!")).start();
            ;
        }
    }

    // Thread.sleep()
    // 실행중인 스레드를 잠시 중단시키는 메서드이다.
    // Interruption이 발생하면 실행중인 스레드가 종료될 수도 있다. (IterruptedExcetion)
    public static class TestThreadSleep {
        public void test() throws InterruptedException {
            String[] msgs = { "Hello,", "Sleepy", "Thread!" };
            for (String s : msgs) {
                Thread.sleep(1000);
                System.out.println(s);
            }
        }
    }

    // Interrupt
    // Thread가 현재 작업을 중단하고 다른 작업을 수행햐아 함을 의미한다.
    public static class TestHandleInterrupt {
        public void test() throws InterruptedException {
            long sum = 0;

            for (int i = 0; i < Integer.MAX_VALUE; i++) {
                sum += i;

                // 만약 스레드에서 IterruptedException을 던지는 메서드가 없이
                // 오랜 시간이 걸리는 작업을 수행하는 경우,
                // 명시적으로 Thread가 interrupt 되었는지 확인하는 코드를 추가해야 합니다.
                if (Thread.interrupted()) {
                    throw new InterruptedException();
                }
            }
        }
    }

    // Join
    // Thread.join(): 다른 스레드가 완료될 때 까지 기다리는 메서드
    public static class TestJoin {

        public void test() throws InterruptedException {
            Thread thread = new Thread(() -> {
                try {
                    Thread.sleep(3000);
                    System.out.println("I'm Lazy"); // First
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            });
            thread.start();
            thread.join();
            System.out.println("I'm Diligent"); // Second

        }
    }

    //
    // Thread의 상태는 NEW, RUNNABLE, BLOCKED, WAITING, TIMED_WAITING, TERMINATED
    // NEW: 스레드가 만들어지고 아직 실행되지 않은 상태
    // RUNNABLE: 스레드가 실행 또는 실행대기 중인 상태
    // BLOCKED: 다른 스레드에 의해 lock된 영역에 접근하려고 기다리는 상태
    // WAITING: I/O를 기다리고 있는 상태
    // TIMED WAITING: timeout 매개변수가 있는 메서드를 호출한 상태.
    // TERMINATED: 쓰레드가 종료된 상태
    public static class TestThreadState {
        public void test() throws InterruptedException {
            Thread parent = new Thread(() -> {
                Thread child = new Thread(() -> {
                    try {
                        Thread.sleep(3000);
                    } catch (InterruptedException e) {
                        e.printStackTrace();
                    }
                });
                System.out.println("1. Child: " + child.getState()); // NEW

                child.start();
                System.out.println("2. Child: " + child.getState()); // RUNNABLE

                try {
                    Thread.sleep(300);
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
                System.out.println("3. Child: " + child.getState()); // TIMED_WAITING

                try {
                    child.join();
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
                System.out.println("4. Child: " + child.getState()); // TERMINATED
            });
            System.out.println("1. Parent: " + parent.getState()); // NEW

            parent.start();
            System.out.println("2. Parent: " + parent.getState()); // RUNNABLE

            Thread.sleep(1000); // Parent thread에서 child thread의 join()을 호출한 상태
            System.out.println("3. Parent: " + parent.getState()); // WAITING

            parent.join();
            System.out.println("4. Parent: " + parent.getState()); // TERMINATED
        }
    }

    // Thread Priority
    // 스레드 스케쥴러는 스레드의 우선순위에 따라 스레드에 프로세서를 할당한다.
    // JVM이 제공하거나, 프로그래머가 명시적으로 선언할 수 있다.
    // 1 ~ 10 사이의 값을 갖는다

    // Thread 클래스는 우선순위 값으로 다음의 static 필드를 가지고 있다.
    // MIN_PRIORITY: 1
    // NORM_PRIORITY: 5
    // MAX_PRIORITY: 10
    public static class TestThreadPriority {

        public void test() {
            Thread th1 = createThread();
            Thread th2 = createThread();
            Thread th3 = createThread();
            th1.setName("Thread 1");
            th2.setName("Thread 2");
            th3.setName("Thread 3");

            printThreadPriority(th1, th2, th3); // 5, 5, 5
            th1.setPriority(Thread.MIN_PRIORITY);
            th2.setPriority(Thread.NORM_PRIORITY);
            th3.setPriority(Thread.MAX_PRIORITY);
            printThreadPriority(th1, th2, th3); // 1, 5, 10

            startThread(th1, th2, th3);
            // 1, 2, 3 순으로 실행
            // 3, 2, 1 순으로 완료
        }

        private Thread createThread() {
            return new Thread(() -> {
                String name = Thread.currentThread().getName();

                System.out.println("[" + name + "]" + " Started");

                long sum = 0;
                for (int i = 0; i < Integer.MAX_VALUE; i++) {
                    sum += i;
                }

                System.out.println("[" + name + "]" + " Finished");
            });
        }

        private void startThread(Thread... threads) {
            for (Thread t : threads) {
                t.start();
            }
        }

        private void printThreadPriority(Thread... threads) {
            for (Thread t : threads) {
                String name = t.getName();
                int priority = t.getPriority();
                System.out.println("[" + name + "]" + " Priority is " + priority);
            }
        }
    }

    // 동기화 (synchronize)
    // final 객체를 사용하면 synchronize 이슈로 부터 자유롭다 (effective java)
    public static class TestSynchronize {
        private Integer counter = 0;

        public synchronized void increase() {
            counter++;
        }

        public void decrease() {
            // Synchronize the whole object
            synchronized (this) {
                this.counter--;
            }
        }

        public int value() {
            // Synchronize the field only
            synchronized (counter) {
                return counter;
            }
        }

        public void test() throws InterruptedException {
            TestSynchronize sync = new TestSynchronize();
            Thread t1 = new Thread(() -> {
                for (int i = 0; i < 100000; i++) {
                    sync.increase();
                }
            });
            Thread t2 = new Thread(() -> {
                for (int i = 0; i < 100000; i++) {
                    sync.increase();
                }
            });

            t1.start();
            t2.start();

            t1.join();
            t2.join();

            assert sync.value() == 0;
        }
    }

    public static void main(String[] args) throws InterruptedException {
        new TestPrintRunnable().test();
        new TestPrintThread().test();
        new TestThreadSleep().test();
        new TestHandleInterrupt().test();
        new TestJoin().test();
        new TestThreadState().test();
        new TestThreadPriority().test();
        new TestSynchronize().test();
    }

}

// References
// https://velog.io/@dion/%EB%B0%B1%EA%B8%B0%EC%84%A0%EB%8B%98-%EC%98%A8%EB%9D%BC%EC%9D%B8-%EC%8A%A4%ED%84%B0%EB%94%94-10%EC%A3%BC%EC%B0%A8-%EB%A9%80%ED%8B%B0%EC%8A%A4%EB%A0%88%EB%93%9C-%ED%94%84%EB%A1%9C%EA%B7%B8%EB%9E%98%EB%B0%8D