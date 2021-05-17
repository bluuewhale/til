# https://programmers.co.kr/learn/courses/30/lessons/43163

from collections import defaultdict

def solution(begin, target, words):
    if not target in words:
        return 0
    
    #words = {w: False for w in words} # hashset
    lv = 0
    lvs = defaultdict(lambda: [])
    lvs[lv].append(begin) # top-level
    
    while len(words):
        upper_lv= lvs[lv]
        lv += 1
        curr_lv = lvs[lv]
        
        for w1 in upper_lv:
            for (i, w2) in enumerate(words):
                
                is_convertable = sum([c1 == c2 for c1, c2 in zip(list(w1), list(w2))]) == (len(w1) -1)
                if is_convertable:
                    if target == w2:
                        return lv
                    
                    del words[i]
                    curr_lv.append(w2)
            
    return lv