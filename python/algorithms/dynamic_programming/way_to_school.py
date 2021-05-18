# https://programmers.co.kr/learn/courses/30/lessons/42898

CACHE = {}

def find_path(m, n):
    key = (m, n)
    if not key in CACHE:
        if (m == 0) or (n == 0):
            CACHE[key] = 0
        else:
            CACHE[key] = find_path(m-1, n) + find_path(m, n-1)  
    return CACHE.get(key)
    
def solution(m, n, puddles):
    CACHE[(1, 1)] = 1
    
    for (x, y) in puddles:
        CACHE[(x, y)] = 0
        
    return find_path(m, n) % 1000000007