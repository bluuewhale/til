from typing import *

def merge(a:List[object], b: List[object]):
    # object should be orderable
    # each array should be sorted

    result = []

    p1 = 0
    p2 = 0

    while True:

        if p1 == len(a): # reached end
            result += b[p2:]
            break

        elif p2 == len(b): # reached end
            result += a[p1:]
            break

        x, y = a[p1], b[p2]

        if x <= y:
            result.append(x)
            p1 += 1
        else:
            result.append(y)
            p2 += 1

    return result
        

def merge_sort(array: List[object]):

    if len(array) == 1:
        return array

    first_half = array[:len(array)//2]
    second_half = array[len(array)//2:]
    return merge(merge_sort(first_half), merge_sort(second_half))


if __name__ == "__main__":

    assert merge([1,2,5], [3, 4, 6]) == [1, 2, 3, 4, 5, 6]
    assert merge_sort([3, 5, 4, 1, 6, 2]) == [1, 2, 3, 4, 5, 6]
