# https://programmers.co.kr/learn/courses/30/lessons/42883

def get_drop_target(nums):
    for (i, (p, n)) in enumerate(zip(nums[:-1], nums[1:])):
        if p < n:
            return i
    return len(nums) - 1
    
def solution(number, k):
    nums = [int(n) for n in number]
    
    while k:
        idx = get_drop_target(nums)
        del nums[idx]
        
        k -= 1
    
    return "".join([str(n) for n in nums])
