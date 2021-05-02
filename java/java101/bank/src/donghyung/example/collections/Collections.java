package donghyung.example.collections;

import java.util.ArrayList;
import java.util.HashMap;

public class Collections {
    public static void main(String[] args) {
        // ArrayList
        ArrayList<String> list = new ArrayList<String>();
        list.add("Hello");
        list.add("C");
        list.add("World");
        list.add(2, "Programming");

        list.set(1, "Java");
        String str = list.get(2);
        System.out.println("str: " + str);

        list.clear();

        boolean isEmpty = list.isEmpty();
        System.out.println("isEmpty: " + isEmpty);

        // HashMap
        HashMap<String, Integer> map = new HashMap<String, Integer>();

        map.put("Tom", 5);
        map.put("Jenny", 10);
        map.put("Mark", 20);
        map.remove("Jenny");

        System.out.println("Mark : " + map.get("Mark"));
        System.out.println("Has Jenny?: " + map.containsKey("Jenny"));
    }
}