import java.lang.ref.WeakReference;
import java.nio.channels.spi.SelectorProvider;
import java.util.HashMap;
import java.util.WeakHashMap;

public class WeakReferences {
    public final static class UniqueId<T> {
        private final Object id;

        public UniqueId(T id) {
            this.id = id;
        }
    }

    public static void main(String[] args) {
        String s = new String("Pick me!");
        WeakReference<String> wr = new WeakReference<String>(s);
        assert s == wr.get();

        // Weak Reference는 참조하고 있는 Strong Refernce가 사라지면, weakly reachable 상태가 된다. 되며,
        // weakly rechable한 상태의 referent는 GC에 의해 제거된다.
        s = null;
        assert wr.get() == null;

        // 내부 캐시를 HashMap를 이용해 구현하는 경우,
        // 실제 데이터가 더이상 사용되지 않을 때 캐시를 함께 삭제해주어야 하는 불편함이 있다.
        // 이 경우, WeakHashMap을 활용하면 매우 유용하다.
        UniqueId<Integer> key = new UniqueId<Integer>(20);
        String val = "Tom";
        WeakHashMap<UniqueId<Integer>, String> registry = new WeakHashMap<UniqueId<Integer>, String>();
        registry.put(key, val);

        key = null;
        while (!(registry.size() == 0)) {
            try {
                System.gc();
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }
        assert registry.isEmpty() == true;
    }
}
