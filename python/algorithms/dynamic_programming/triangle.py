# https://programmers.co.kr/learn/courses/30/lessons/43105?language=python3

from collections import defaultdict

def solution(triangle):
    reversed_triangle = triangle[::-1]
    max_path = defaultdict(lambda: [])
    
    for (i, lv) in enumerate(reversed_triangle):
        if i == 0:
            max_path[i] = lv
            continue
    
        for (j, node) in enumerate(lv):
            maxp1, maxp2 = max_path[i-1][j], max_path[i-1][j+1] # parents
            max_new = node + max(maxp1, maxp2)
            
            max_path[i].append(max_new)
            
    return max_path[len(triangle) -1].pop()