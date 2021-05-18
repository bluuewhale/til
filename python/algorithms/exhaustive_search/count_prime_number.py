# https://programmers.co.kr/learn/courses/30/lessons/42839?language=python3
from itertools import permutations

def is_prime(n):
    if n <= 1:
        return False
    
    i = 1
    while True:
        i += 1
        if i ** 2 > n:
            break
        
        if n % i == 0:
            return False
        
    return True
        
def solution(numbers):
    numbers = [n for n in numbers]
    
    nums = []
    for i in range(1, len(numbers)+1):
        nums.extend([int("".join(x)) for x in permutations(numbers, i)])
    
    nums = list(set(nums))
    print(nums)
    answer = sum([is_prime(n) for n in nums])
    
    return answer