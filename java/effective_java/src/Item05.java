import java.util.List;
import java.util.Objects;
import java.util.function.Supplier;

import org.xml.sax.ext.LexicalHandler;

import Item05.Dictionary.KoreanDictionary;

// Item05: Prefer dependency injection to hardwiring resources

// SpellChecker: 스펠링이 맞는지 확인하는 유틸리티 class
// Dictionary: 사전 클래스
// SpellChecker 클래스는 Dictionary를 사용(의존)하여 기능을 수행한다.
public class Item05 {

    public static class SpellChecker {
        private final Dictionary dict;

        public SpellChecker(Dictionary dict) {
            this.dict = Objects.requireNonNull(dict);
        }

        // instance가 아닌 factory를 주입하는 방식도 유용하다.
        public SpellChecker(Supplier<? extends Dictionary> factory) {
            this.dict = Objects.requireNonNull(factory).get();
        }

        public boolean isValid(String word) {
            this.dict.print();
            return true;
        }

        public List<String> sugguestions(String word) {
            throw new UnsupportedOperationException();
        }
    }

    public interface Dictionary {
        public void print();

        public class KoreanDictionary implements Dictionary {
            @Override
            public void print() {
                System.out.println("korean");
            }

        }

        public class EnglishDictionary implements Dictionary {

            @Override
            public void print() {
                System.out.println("english");
            }
        }
    }

    public static void main(String[] args) {
        Dictionary krDict = new Dictionary.KoreanDictionary();
        Dictionary engDict = new Dictionary.EnglishDictionary();
        SpellChecker krSpellChecker = new SpellChecker(krDict);
        SpellChecker engSpellChecker = new SpellChecker(engDict);
        SpellChecker krSpellChecker_ = new SpellChecker(() -> krDict);

        krSpellChecker.isValid("안녕");
        engSpellChecker.isValid("hi");
    }
}
