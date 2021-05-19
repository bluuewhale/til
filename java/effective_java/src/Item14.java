import java.text.CollationElementIterator;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Collections;
import java.util.List;
import java.util.Set;
import java.util.TreeSet;
import java.util.stream.Collectors;

// Item14: Consider implementing Comparable

// compareTO() method is the sole method in the Comparable interface.
// By implementing Comparable, a class indicates that its instance have a natural ordering

public class Item14 {

    // By implementing Comparable, you allow your class to iteroperate
    // with all of the many generic algorithms and collections implementations
    // that depend on this interface, such as BST, Heap, sorting algorithms;
    public static final class CaseInsensitiveString implements Comparable<CaseInsensitiveString> {
        private final String s;

        public CaseInsensitiveString(String s) {
            this.s = s;
        }

        @Override
        public int compareTo(Item14.CaseInsensitiveString o) {
            return s.toLowerCase().compareTo(o.s.toLowerCase());
        }

        @Override
        public String toString() {
            return this.s;
        }
    }

    public static class WordList {
        private Set<CaseInsensitiveString> set;

        public WordList(String[] strs) {
            // List<String> strs_ = Arrays.asList(strs);
            List<CaseInsensitiveString> strings = Arrays.asList(strs).stream().map(CaseInsensitiveString::new)
                    .collect(Collectors.toList());

            Set<CaseInsensitiveString> set = new TreeSet<>();
            Collections.addAll(set, strings.toArray(new CaseInsensitiveString[strings.size()]));

            this.set = set;
        }

        public void print() {
            System.out.println(set);
        }
    }

    public static void main(String[] args) {
        String[] inputs = { "c", "A", "B" };
        WordList wordList = new WordList(inputs);
        wordList.print(); // [A, B, c]
    }
}
