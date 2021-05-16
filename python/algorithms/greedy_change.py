from collections import Counter

def solution(string):
    chars = []
    num = 0
    for s in string:
        if s.isdecimal():
            num += int(s)
        else:
            chars.append(s)

    chars_counter = Counter(chars)
    chars_sorted = "".join([k * v for (k, v) in sorted(chars_counter.items())])
    return "".join([chars_sorted, str(num)])

if __name__ == "__main__":
    assert solution("K1KA5CB7") == "ABCKK13"
