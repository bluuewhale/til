import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedList;
import java.util.Objects;
import java.util.Queue;
import java.util.Vector;

public class GraphConnectionBetweenNodes {
    static class Graph {
        public static class Node {
            int val;
            LinkedList<Node> adjcent;
            boolean isVisited;

            public Node(int val) {
                this.val = val;
                this.adjcent = new LinkedList<Node>();
                this.isVisited = false;
            }

            public void connect(Node n) {
                if (!(adjcent.contains(n))) {
                    adjcent.add(n);
                }
                if (!(n.adjcent.contains(this))) {
                    n.adjcent.add(this);
                }
            }
        }

        ArrayList<Node> nodes;

        public Graph() {
            this.nodes = new ArrayList<Node>();
        }

        public Graph(int size) {
            Node[] nodes = new Node[size];
            for (int i = 0; i < size; i++) {
                nodes[i] = new Node(i);
            }
            this.nodes = new ArrayList<>(Arrays.asList(nodes));
        }

        public void connect(int i1, int i2) {
            System.out.println("Connect " + i1 + " To " + i2);
            Node start = nodes.get(i1);
            Node end = nodes.get(i2);

            if (!(start == null) && !(end == null)) {
                start.connect(end);
            }
        }

        void initVisit() {
            for (Node n : nodes) {
                n.isVisited = false;
            }
        }

        public boolean isConnected(int i1, int i2) {
            initVisit();

            Node start = Objects.requireNonNull(nodes.get(i1));
            Node end = Objects.requireNonNull(nodes.get(i2));

            Queue<Node> queue = new LinkedList<>();
            queue.add(start);
            while (!(queue.isEmpty())) {
                Node n = queue.poll();

                if (!(n.isVisited)) {
                    n.isVisited = true;

                    if (n.equals(end)) {
                        return true;
                    }

                    for (Node adj : n.adjcent) {
                        queue.add(adj);
                    }
                }
            }

            return false;
        }

    }

    public static Graph createSample() {
        // letter '.' refers to black space due to vscode auth forammting
        // 0
        // |
        // 1 -- 3 -- 5 -- 7
        // | ./ | ..
        // 2 -- 4 .. 6 -- 8

        Graph graph = new Graph(10);
        graph.connect(0, 1);
        graph.connect(1, 3);
        graph.connect(2, 3);
        graph.connect(1, 2);
        graph.connect(2, 4);
        graph.connect(3, 5);
        graph.connect(5, 7);
        graph.connect(6, 8);

        return graph;
    }

    public static void main(String[] args) {
        Graph graph = createSample();

        assert graph.isConnected(0, 4);
        assert graph.isConnected(1, 4);
        assert graph.isConnected(2, 7);
        assert !graph.isConnected(4, 8);
    }
}
