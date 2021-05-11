import java.util.LinkedList;
import java.util.Queue;
import java.util.Stack;

public class Graph {
    Node[] nodes;

    public static class Node {
        int val;
        LinkedList<Node> adjcent;
        boolean isVisited;

        public Node(int val) {
            this.val = val;
            this.adjcent = new LinkedList<Node>();
            this.isVisited = false;
        }

        public void connect(Node node) {
            this.adjcent.add(node);
        }

        public boolean isConnected(Node node) {
            return this.adjcent.contains(node);
        }
    }

    public Graph(int size) {
        this.nodes = new Node[size];
        for (int i = 0; i < size; i++) {
            this.nodes[i] = new Node(i);
        }
    }

    public void connect(int idxA, int idxB) {
        Node n1 = nodes[idxA];
        Node n2 = nodes[idxB];

        if (!n1.isConnected(n2)) {
            n1.connect(n2);
        }
        if (!n2.isConnected(n1)) {
            n2.connect(n1);
        }
    }

    public void visit(Node node) {
        System.err.println(node.val);
        node.isVisited = true;
    }

    public void dfs() {
        Stack<Node> stack = new Stack<Node>();

        Node root = this.nodes[0];
        stack.add(root); // add root
        while (!(stack.isEmpty())) {
            Node node = stack.pop();

            if (!(node.isVisited)) {
                visit(node);

                for (Node n : node.adjcent) {
                    stack.add(n);
                }
            }

        }
    }

    public void bfs() {
        Queue<Node> queue = new LinkedList<>();

        Node root = this.nodes[0];
        queue.add(root); // add root
        while (!(queue.isEmpty())) {
            Node node = queue.poll();

            if (!(node.isVisited)) {
                visit(node);

                for (Node n : node.adjcent) {
                    queue.add(n);
                }
            }

        }
    }

    public void dfsR() {
        Node root = nodes[0];
        dfsR(root);
    }

    private void dfsR(Node r) {
        if (r == null)
            return;

        if (!(r.isVisited)) {
            visit(r);
            for (Node n : r.adjcent) {
                dfsR(n);
            }
        }
    }

    private void dfsR(int idx) {
        Node node = nodes[idx];
        dfsR(node);
    }

    public static Graph createSample() {
        // letter '.' refers to black space due to vscode auth forammting
        // 0
        // |
        // 1 -- 3 -- 5 -- 7
        // | ./ | .. |
        // 2 -- 4 .. 6 -- 8

        Graph graph = new Graph(10);
        graph.connect(0, 1);
        graph.connect(1, 3);
        graph.connect(2, 3);
        graph.connect(1, 2);
        graph.connect(2, 4);
        graph.connect(3, 5);
        graph.connect(5, 6);
        graph.connect(6, 8);
        graph.connect(5, 7);

        return graph;
    }

    public static void main(String[] args) {
        Graph.createSample().dfs(); // 0 1 2 4 3 5 7 6 8
        Graph.createSample().bfs(); // 0 1 3 2 5 4 6 7 8
        Graph.createSample().dfsR(); // 0 1 3 2 4 5 6 8 7
        Graph.createSample().dfsR(8); // 8 6 5 3 1 0 2 4 7
    }
}
