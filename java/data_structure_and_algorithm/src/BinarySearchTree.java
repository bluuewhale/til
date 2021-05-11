public class BinarySearchTree {
    enum Ordering {
        GREATER, EQUAL, LESS,
    }

    class Node {
        final int val;
        Node left;
        Node right;

        Node(int val) {
            this.val = val;
        }

        void add(Node n) {
            if (val > n.val) {
                addLeft(n);
            } else if (val < n.val) {
                addRight(n);
            }
        }

        void addLeft(Node n) {
            if (left == null) {
                left = n;
            } else {
                left.add(n);
            }
        }

        void addRight(Node n) {
            if (right == null) {
                right = n;
            } else {
                right.add(n);
            }
        }
    }

    Node root;

    public BinarySearchTree() {
        this.root = null;
    }

    public void add(int val) {
        Node node = new Node(val);

        if (this.root == null) {
            this.root = node;
            return;
        }

        root.add(node);
    }

    void visit(Node n) {
        System.out.println(n.val);
    }

    public void traversePreOrder() {
        traversePreOrder(root);
    }

    private void traversePreOrder(Node n) {
        if (n == null) {
            return;
        }
        visit(n);
        traversePreOrder(n.left);
        traversePreOrder(n.right);
    }

    public void traverseInOrder() {
        traverseInOrder(root);
    }

    private void traverseInOrder(Node n) {
        if (n == null) {
            return;
        }
        traverseInOrder(n.left);
        visit(n);
        traverseInOrder(n.right);
    }

    public void traversePostOrder() {
        traversePostOrder(root);
    }

    private void traversePostOrder(Node n) {
        if (n == null) {
            return;
        }
        traversePostOrder(n.left);
        traversePostOrder(n.right);
        visit(n);
    }

    public static BinarySearchTree createSample() {
        // ........ 20
        // ...... / .. \
        // .... 10 .... 30
        // ... /. \ .. / .\
        // .. 5 .. 15 25 . 40
        BinarySearchTree tree = new BinarySearchTree();
        tree.add(20);
        tree.add(10);
        tree.add(30);
        tree.add(5);
        tree.add(15);
        tree.add(25);
        tree.add(40);

        return tree;
    }

    public static void main(String[] args) {
        BinarySearchTree tree = BinarySearchTree.createSample();
        tree.traversePreOrder(); // 20, 10, 5, 15, 30, 25, 40
        tree.traverseInOrder(); // 5, 10, 15, 20, 25, 30, 40
        tree.traversePostOrder(); // 5, 15, 10, 25, 40, 30, 20
    }
}
