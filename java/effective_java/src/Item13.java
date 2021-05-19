import java.util.Arrays;
import java.util.EmptyStackException;

// Item13: Override clone judiciously

// The Clonable interface was intended as a mixin interface
// for classes to advertise that they permit cloning
// However, it lacks a `clone` method, and Object's
// clone method is protected

// Then, what does `Clonable` interface do?
// It decides how `clone` (protected) method in Object
// actually behaves

// SUMMARY
// Avoid implementing Cloneable.
// A better approach to object copying is to provide a copy constructor 
// or copy factory
public class Item13 {
    public static class Stack<T> implements Cloneable {
        private Object[] elements;
        private int size = 0;
        private static final int DEFAULT_CAPACITY = 16;

        public Stack() {
            this.elements = new Object[DEFAULT_CAPACITY];
        }

        // Copy Constructor (a better approach)
        public Stack(Stack<T> stack) {
            this.elements = stack.elements.clone();
            this.size = stack.size;
        }

        // Copy Factory Method (a better approach)
        public static <T> Stack<T> newInstance(Stack<T> stack) {
            return new Stack<T>(stack);
        }

        public void push(Object e) {
            ensureCapacity();
            elements[size++] = e;
        }

        public T pop() {
            if (size == 0) {
                throw new EmptyStackException();
            }
            T result = (T) elements[--size];
            elements[size] = null; // remove obsolete reference (memory leak)
            return result;
        }

        private void ensureCapacity() {
            if (elements.length == size) {
                elements = Arrays.copyOf(elements, 2 * elements.length + 1);
            }
        }

        // If an object contains fields that refer to mutable
        // objects, the simple clone implementation can be disastrous

        // In effect, the `clone` method functions as a constructor.
        // You must ensure that it does no harm to the original obejct
        // and that is properly establishes invariants on the clone

        // Public clone methods should omit the throws clause
        // as methods that don’t throw checked exceptions are easier to use (Item 71)
        @Override
        public Stack<T> clone() {
            try {
                Stack<T> result = (Stack<T>) super.clone();
                result.elements = elements.clone(); // MUST COPY INTERNALS
                return result;
            } catch (CloneNotSupportedException e) {
                throw new AssertionError();
            }
        }
    }
}
