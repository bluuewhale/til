# https://programmers.co.kr/learn/courses/30/lessons/43162
from collections import deque

class Node:
    def __init__(self, idx):
        self.idx = idx
        self.adjcent = {}
        self.is_visited = False
    
    def link(self, node):
        self.adjcent[node.idx] = node
    
    def is_linked(self, idx):
        return idx in self.adjcent
    
    def get_linked_nodes(self):
        return list(self.adjcent.values())
    
    def bfs(self):
        result = []
        
        queue = deque()
        queue.append(self)
        
        while len(queue):
            node = queue.popleft()
            
            if not node.is_visited:
                node.is_visited = True
                result.append(node)
                
                for n in node.get_linked_nodes():
                    if not n.is_visited:
                        queue.append(n)
                
        
        return result

class Graph:
    def __init__(self, size):
        self.nodes = {i: Node(i) for i in range(size)}
    
    def get_node(self, idx):
        return self.nodes.get(idx, None)
    
    def link(self, idx1, idx2):
        n1 = self.nodes.get(idx1)
        n2 = self.nodes.get(idx2)
        
        if (n1 is None) or (n2 is None):
            return
        
        if not n1.is_linked(idx2):
            n1.link(n2)
        
        if not n2.is_linked(idx1):
            n2.link(n1)
        
def solution(n, computers):
    graph = Graph(n)
    
    # create links
    for i in range(n):
        for j in range(i +1 , n):
            if computers[i][j] == 1:
                graph.link(i, j)
    
    # count number of networks
    count = 0
    
    nodes_to_go = {i: True for i in range(n)}
    while len(nodes_to_go):
        count += 1
        
        idx = list(nodes_to_go.keys())[0]
        node = graph.get_node(idx)
        for n in node.bfs():
            nodes_to_go.pop(n.idx, None)
    
    return count