''' https://leetcode.com/explore/featured/card/fun-with-arrays/527/searching-for-items-in-an-array/3251/


Given an array A of integers, return true if and only if it is a valid mountain array.

Recall that A is a mountain array if and only if:

A.length >= 3
There exists some i with 0 < i < A.length - 1 such that:
A[0] < A[1] < ... A[i-1] < A[i]
A[i] > A[i+1] > ... > A[A.length - 1]


Example 1:

Input: [2,1]
Output: false
Example 2:

Input: [3,5,5]
Output: false
Example 3:

Input: [0,3,2,1]
Output: true
'''

class Solution:
    def validMountainArray(self, A: List[int]) -> bool:
        if len(A) < 3:
            return False

        reached_peak = False

        for i, (v1, v2) in enumerate(zip(A[:-1], A[1:])):
            if v1 == v2: # no peak
                return False
            
            if not reached_peak:
                if (v1 > v2):
                    if i == 0: # descending only
                        return False
                    
                    reached_peak = True
            else:
                if v1 < v2:
                    return False
        
        return True & reached_peak

