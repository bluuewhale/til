''' https://leetcode.com/explore/featured/card/fun-with-arrays/511/in-place-operations/3157/

Given an array nums, write a function to move all 0's to the end of it while maintaining the relative order of the non-zero elements.

Example:

Input: [0,1,0,3,12]
Output: [1,3,12,0,0]
Note:

You must do this in-place without making a copy of the array.
Minimize the total number of operations.

'''

class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        curr_ptr = 0
        for i, v in enumerate(nums):
            if v == 0:
                continue
            
            nums[curr_ptr] = v
            curr_ptr += 1
        
        for j in range(curr_ptr, len(nums)):
            nums[j] = 0