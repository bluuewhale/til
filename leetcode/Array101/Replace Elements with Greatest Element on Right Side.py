''' https://leetcode.com/explore/featured/card/fun-with-arrays/511/in-place-operations/3259/

Given an array arr, replace every element in that array with the greatest element among the elements to its right, and replace the last element with -1.

After doing so, return the array.

 

Example 1:

Input: arr = [17,18,5,4,6,1]
Output: [18,6,6,6,1,-1]
 

Constraints:

1 <= arr.length <= 10^4
1 <= arr[i] <= 10^5
'''

class Solution:
    def replaceElements(self, arr: List[int]) -> List[int]:
        if len(arr) == 1:
            arr[0] = -1
            return arr

        if len(arr) == 2:
            arr[-2] = arr[-1]
            arr[-1] = -1
            return arr
        
        max_:int = 0
        for i in range(-1, -(len(arr)+1), -1): # reverse iteration
            if i == -1: # replace last element at the end
                continue
            
            if arr[i+1] > max_:
                max_ = arr[i+1]
            
            # remember current value
            v = arr[i]

            # replace
            arr[i] = max_

            # check if current value can be a new max
            if v > max_:
                max_ = v
        
        arr[-1] = -1
        return arr

            


            
            

                



        
        