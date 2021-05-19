import java.util.Objects;

// Item 12: Always override toString

// prividing a good toString() implementation makes your
// class much more pleasant to use and makes system
// using the class easier to debug

// When practical, the toString() method should return all of 
// the interesting information contained in the object

// Whether or not you decide to specify the format,
// you should clearly document your intentions

// provide programmatic access to the information contained in
// the return by toString()
public class Item12 {

    static final class PhoneNumber {
        private final int areaCode;
        private final int prefix;
        private final int lineNum;

        public PhoneNumber(int areaCode, int prefix, int lineNum) {
            this.areaCode = Objects.requireNonNull(areaCode);
            this.prefix = Objects.requireNonNull(prefix);
            this.lineNum = Objects.requireNonNull(lineNum);
        }

        // This method will return informations of the instance
        // with format "XXX-YYY-ZZZZ"
        // XXX: areaCode
        // YYY: prefix
        // ZZZZ: lineNum
        @Override
        public String toString() {
            return String.format("%03d-%03d-%04d", areaCode, prefix, lineNum);

        }

        public int getAreaCode() {
            return this.areaCode;
        }

        public int getPrefix() {
            return this.prefix;
        }

        public int getLineNum() {
            return this.lineNum;
        }

        @Override
        public int hashCode() {
            int result = Integer.hashCode(areaCode);
            result = 31 * result + Integer.hashCode(prefix);
            result = 31 * result + Integer.hashCode(lineNum);
            return result;
        }
    }

    public static void main(String[] args) {
        PhoneNumber jenny = new PhoneNumber(707, 867, 5309);
        System.out.println("Jenny's Phone Number: " + jenny);
    }
}
