''' https://leetcode.com/explore/featured/card/fun-with-arrays/525/inserting-items-into-an-array/3245/

Given a fixed length array arr of integers, duplicate each occurrence of zero, shifting the remaining elements to the right.

Note that elements beyond the length of the original array are not written.

Do the above modifications to the input array in place, do not return anything from your function.

 

Example 1:

Input: [1,0,2,3,0,4,5,0]
Output: null
Explanation: After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]
Example 2:

Input: [1,2,3]
Output: null
Explanation: After calling your function, the input array is modified to: [1,2,3]
 

Note:

1 <= arr.length <= 10000
0 <= arr[i] <= 9

'''
import copy

class Solution:
    def duplicateZeros(self, arr: List[int]) -> None:
        """
        Do not return anything, modify arr in-place instead.
        """
        original_len = len(arr)
        arr_copy = copy.deepcopy(arr)

        while len(arr):
            arr.pop()
        
        for v in arr_copy:
            arr.append(v)        

            if len(arr) == original_len:
                break

            if v == 0:
                arr.append(0) # dupl zero

            if len(arr) == original_len:
                break

            