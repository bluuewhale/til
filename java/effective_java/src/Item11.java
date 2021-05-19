// Always override hashCode when you override equals

// hashCode() method must consistently return the same value
// if two objects are equal, they should return same hashCode() result

// The key provision that is violated when you fail to override hashCode
// is the second one: equal object must have equal hash codes.

public class Item11 {

    public static class PhoneNumber {
        final Short areaCode;
        final Short prefix;
        final Short lineNum;
        private int hashCode = 0; // lazy initialization

        public PhoneNumber(Short areaCode, Short prefix, Short lineNum) {
            this.areaCode = areaCode;
            this.prefix = prefix;
            this.lineNum = lineNum;
        }

        // Good Example of hashCode
        // a good hashCode should spread result uniformly
        // 31 is a odd/prime number
        // multiplication with 31 can be replaced with shift and subtract operations
        // which is (i << 5) - i
        @Override
        public int hashCode() {
            int result = hashCode;
            if (result == 0) { // lazy initialization
                result = Short.hashCode(areaCode);
                result = 31 * result + Short.hashCode(prefix);
                result = 31 * result + Short.hashCode(lineNum);
                hashCode = result;
            }
            return result;
        }
    }

}
