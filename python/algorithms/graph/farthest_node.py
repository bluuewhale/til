# https://programmers.co.kr/learn/courses/30/lessons/49189?language=python3

from collections import defaultdict, deque

class Node:
    def __init__(self, idx):
        self.idx = idx
        self.is_visited = False
        self.adjcent = {}
    
    def link(self, node):
        self.adjcent[node.idx] = node
    
    def is_linked_to(self, idx):
        return idx in self.adjcent
    
    def get_linked_nodes(self):
        return list(self.adjcent.values())
        
        
class Graph:
    
    def __init__(self, size):
        self.nodes = {i+1: Node(i+1) for i in range(size)}
        
    def link(self, i1, i2):
        n1, n2 = self.nodes.get(i1), self.nodes.get(i2)
        if (n1 is None) or (n2 is None):
            return
        
        n1.link(n2)
        n2.link(n1)
        
    def bfs(self):
        lv = 1
        lvs = {}
        lvs[1] = {
            1: self.nodes.get(1)
        }
        
        while True:
            old = lvs[lv] # upper level
            new = {} # current level            
            for (i, n) in old.items():
                if not n.is_visited:
                    n.is_visited = True

                    adj_nodes = n.get_linked_nodes()
                    for adj in adj_nodes:
                        if (not adj.is_visited) and (not adj.idx in old):
                            new[adj.idx] = adj
            if not new:
                break
            lv += 1
            lvs[lv] = new
            
        return lvs
        
def solution(n, edge):
    graph = Graph(n)
    for (i1, i2) in edge:
        graph.link(i1, i2)
    
    lvs = graph.bfs()
    max_lv = max(lvs.keys())
    return len(lvs.get(max_lv))
