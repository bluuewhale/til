'''https://leetcode.com/explore/featured/card/fun-with-arrays/521/introduction/3238/


Given a binary array, find the maximum number of consecutive 1s in this array.

Example 1:
Input: [1,1,0,1,1,1]
Output: 3
Explanation: The first two digits or the last three digits are consecutive 1s.
    The maximum number of consecutive 1s is 3.
Note:

The input array will only contain 0 and 1.
The length of input array is a positive integer and will not exceed 10,000
'''

class Solution:
    def findMaxConsecutiveOnes(self, nums: List[int]) -> int:
        max_cnt:int = 0
        curr_cnt:int = 0

        for n in nums:
            if n:
                curr_cnt += 1
            else:
                curr_cnt = 0
                
            if curr_cnt > max_cnt:
                max_cnt = curr_cnt
        
        return max_cnt
