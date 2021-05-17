# https://programmers.co.kr/learn/courses/30/lessons/43165

from collections import deque, Counter

def solution(numbers, target):
    results = deque([0])
    
    for n in numbers:
        for _ in range(len(results)):
            s = results.popleft()
            results.append(s + n)
            results.append(s - n)
        
    answer = Counter(results).get(target, 0)
    return answer