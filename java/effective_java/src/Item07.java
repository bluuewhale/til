import java.util.Arrays;
import java.util.EmptyStackException;

// Item 07. Eliminate obsolete object references

// [ Tips ]
// 1. 사용자가 직접 메모리를 관리하는 경우, 유효하지 않는 포인터는 제거하여 Memory Leak을 막아야 한다.
// 2. Weak Reference를 활용하면 Memory Leak을 효과적으로 방지할 수 있다.
public class Item07 {

    public static class Stack<T> {
        private static final int DEFAULT_CAPACITY = 16;
        private Object[] elements;
        private int size = 0;

        private Stack(int capactiy) {
            this.elements = new Object[capactiy];
        }

        public static Stack newInstance() {
            return new Stack(DEFAULT_CAPACITY);
        }

        public static Stack withCapacity(int capacity) {
            return new Stack(capacity);
        }

        public void push(T e) {
            ensureCapacity();
            elements[size++] = e;
        }

        public T pop() {
            if (size == 0) {
                throw new EmptyStackException();
            }
            T result = (T) elements[--size];
            elements[size] = null; // Memory leak will ocuur without this line
            return result;
        }

        // Grow double when number of elements reaches the capacity
        private void ensureCapacity() {
            if (elements.length == size) {
                elements = Arrays.copyOf(elements, 2 * size);
            }
        }
    }

    public static void main(String[] args) {
        Stack<Integer> stack = Stack.withCapacity(4);
        stack.push(10);
        stack.push(20);
        stack.push(30);
        assert stack.pop() == 30;
        assert stack.pop() == 20;
        stack.push(40);
        assert stack.pop() == 40;
    }

}
