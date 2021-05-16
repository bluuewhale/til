> 이 글은 `동빈나` 님의 `(이코테 2021 강의 몰아보기) 2. 그리디 & 구현` 영상을 보고 정리한 내용입니다. 원본 영상은 [이 곳](https://www.youtube.com/watch?v=2zjoKjt97vQ&list=PLRx0vPvlEmdAghTr5mXQxGpHjWqSz0dgC&index=2)에서 확인하실 수 있습니다. 

# Greedy Algorithm

그리디 알고리즘(탐욕법)은 현재 상황에서 최선의 선택지를 고르는 방법으로 최적해를 찾는 알고리즘이다. (이름 그대로 욕심쟁이 알고리즘이다). 

현재 상황에서 가장 좋은 선택지를 고르는 것이 항상 최적해를 보장하는 것은 아니다. 따라서, 그리디 해법은 정당성 분석이 중요하다.

&nbsp;

## [문제 상황]: 트리 경로의 합
루트 노드부터 시작하여 거쳐 가는 노드 값의 합을 최대로 만들고 싶습니다.

![](images/2021-05-14-23-56-40.png)

&nbsp;

**Q. 이 문제는 그리디 알고리즘으로 풀 수 있나?**
A. 풀 수 없다. 이 경우 그리디 알고리즘은 최적 해를 보장하지 않는다. 단순한 예로, 잎사귀 노드가 극단적인 값(ex, 1000000)을 가지면 앞의 경로는 별 의미가 없다.  트리 구조에서 각각의 잎사귀 노드에 도달하는 경로는 단 하나만 존재하기 때문에, 모든 잎사귀 노드에 대해 거쳐간 노드 값의 합을 계산하면 이 문제를 해결할 수 있다. 

&nbsp;

## [문제 상황]: 거스름 돈
500원, 100원, 50원, 10원 짜리 동전을 사용하여 잔돈을 거슬러 주어야 한다. 가장 최소의 동전 개수로 거슬러 줄 수 있는 방법을 사용할 때 필요한 동전 개수는?

### 정당성 분석
**Q. 가장 큰 화폐 단위부터 돈을 거슬러 주는 것이 최적의 해를 보장하는 이유는 무엇일까?**

A. 가지고 있는 동전 중에서 큰 단위가 항상 작은 단위의 배수이므로 작은 단위의 동전들을 종합해 다른 해가 나올 수 없기 때문이다. 만약 800원을 거슬러 주어야 하는데 화폐 단위가 500원, 400원, 100원 이라면, 그리디 알고리즘이 최적 해를 보장하지 않게 된다.

### 풀이
```python3

n = 1260
count = 0

coin_types = [500, 100, 50, 10]
for c in coin_types:
    count += n // coin
    n %= coin
```

### 시간 복잡도
위 알고리즘의 시간 복잡도는 금액과는 무관하며, 동전의 총 종류(K)에만 영향을 받으므로, O(K)의 시간 복잡도를 갖는다.

&nbsp;

## [문제 상황] 1이 될 때까지
어떠한 수 N이 1이 될 때까지 다음의 두 과정 중 하나를 반복적으로 선택하여ㅎ수행하려고 합니다. 단, 두번째 연산은 N이 K로 나누어 떨어질 때만 선택할 수 있습니다.

1. N에서 1을 뺍니다.
2. N을 K로 나눕니다.

### 예시
N=17, K=4인 경우, 다음의 과정을 거치면 총 3번의 연산으로 N을 1로 만들 수 있다.
1. N(16) = N(17) - 1 (1번 연산)
2. N(4) = N(16) / K(4) (2번 연산)
3. N(1) = N(4) / K(4) (2번 연산)

### 풀이
``` python3
def solution(n, k):
    cnt = 0
    while n != 1:
        if not (n % k) == 0:
            n -= 1
        else:
            n = n // k
        
        cnt += 1

    return cnt
```

&nbsp;

## [문제 상황]: 곱하기 혹은 더하기
각 자리가 숫자(0부터 9)로만 이루어진 문자열 S가 주어졌을 때, 왼쪽부터 오른쪽으로 하나씩 모든 숫자를 확인하며 숫자 사이에 'x' 혹은 "+" 연산자를 넣어 결과적으로 만들어질 수 있는 가장 큰 수를 구하는 프로그램을 작성하세요

단, 모든 연산은 왼쪽부터 순서대로 이루어진다고 가정합니다.

### 풀이
``` python3
def solution(number):
    sum = 0
    
    for n in number:
        n = int(n)
        if (sum == 0) or (n == 0):
            sum += n
        else:
            sum *= n

    return sum


if __name__ == "__main__":
    assert solution("02984") == 576
    assert solution("328") == 48
```

&nbsp;

## [문제 상황]: 모험가 길드
한 마을에 모험가가 N명이 있습니다. 모험가 길드에서는 N명의 모험가를 대상으로 '공포도'를 측정했는데, 공포도가 높은 모험가는 쉽게 공포를 느껴 위험 상황에서 제대로 대처할 능력이 떨어집니다.

모험가 길드장은 모함가 그룹을 안전하게 구성하고자, 공포도가 X인 모험가는 반드시 X명 이상으로 구성한 모험가 그룹에 참여해야 여행을 떠날 수 있도록 규정하였습니다.

N명의 모험가에 대한 정보가 주어졌을 때, 여행을 떠날 수 있는 그룹 수의 최대값을 구하는 프로그램을 작성하세요.

``` python3
from collections import Counter

def solution(array):
    n = 0 # 모험을 떠날 수 있는 그룹의 수
    counter = Counter(array)

    l = 0 # 직전 그룹을 만들고 남은 사람들
    for (k, v) in sorted(counter.items()):
        n += (v + l) // k
        l = (v + l) % k
    return n

if __name__ == "__main__":
    assert solution([2, 3, 1, 2, 2]) == 2
    assert solution([1, 2, 3, 4, 5]) == 1
    assert solution([1, 1, 2, 2, 3]) == 3
```

&nbsp;

## [문제 상황] 상하좌우

여행가 A는 N x N 크기의 정사각형 공간 위에 서 있습니다. 이 공간은 1 x 1 크기의 정사각형으로 나누어져 있습니다. 가장 왼쪽 위 좌표는 (1,1)이며, 가장 오른쪽 아래 좌표는 (N, N)에 해당합니다. 여행가 A는 상, 하, 좌, 우 방향으로 이동할 수 있으며, 시작 좌표는 항상 (1, 1)입니다. 우리 앞에는 여행가 A가 이동할 계획이 적힌 계획서가 높여 있습니다.

계획서에서 하나의 줄에 띄어쓰기를 기준으로 하여, L, R, U, D, 중 하나의 문자가 반복적으로 적혀 있습니다. 각 문자의 의미는 다음과 같습니다.

L: 왼쪽으로 한 칸 이동
R: 오른쪽으로 한 칸 이동
U: 위로 한 칸 이동
D: 아래로 한 칸 이동

주어진 계획서에 따라 이동을 마쳤을 때, 여행자의 현재 좌표(X, Y)를 공백을 기준으로 구분하여 반환하시오.

이 때, 여행가 A가 N x N 크기의 정사각형 공간을 벗어나는 움직임은 무시됩니다.

### 풀이

``` python3
def solution(n, directions):
    x = 1
    y = 1

    drs = directions.split(" ")
    for d in drs:
        if d == "L":
            if x > 1:
                x -= 1
        if d == "R":
            if x < n:
                x += 1
        if d == "U":
            if y > 1:
                y -= 1
        if d == "D":
            if y < n:
                y += 1

    print(x, y)

    return "{} {}".format(x, y)

if __name__ == "__main__":
    assert solution(5, "R R R R R R") == "5 1"
    assert solution(5, "L L L L L L") == "1 1"
    assert solution(5, "U U U U U U") == "1 1"
    assert solution(5, "D D D D D D") == "1 5"
    assert solution(5, "R R R U D D") == "4 3"
```

&nbsp;

## [문제 상황]: 3이 포함된 시간
정수 N이 입력되면 00시 00분 00초부터 N시 59분 59초까지의 모든 시각 중에서 3이 하나라도 포함되는 모든 경우의 수를 구하는 프로그램을 작성하세요.

예를 들어, 1을 입력했을 때, 다음은 3이 하나라도 포함되어 있으므로 세어야 하는 시각입니다.

### 풀이
```python3

def solution(n):
    sum = 0
    for i in range(n + 1):
        sum += three_in_hour(i)
    return sum

def has_three(x):
    flag = False
    for i in str(x):
        if i == "3":
            flag = True
            break

    return flag
def three_in_hour(h):
    if has_three(h):
        return 3600
    else:
        return sum([three_in_minute(m) for m in range(60)])

def three_in_minute(m):
    if has_three(m):
        return 60
    return 15

if __name__ == "__main__":
    assert solution(5) == 11475
```

&nbsp;

## [문제 상황] 왕실의 나이트
8 x 8 좌표 평면상에서 나이트의 위치가 주어졌을 때, 나이트가 이동할 수 있는 경우의 수를 출력하는 프로그램을 작성하세요. 왕실의 정원에서 행 위치를 표현할 때는 1부터 8로 표현하며, 열 위치를 표현할 때는 a부터 h로 표현합니다.

### 풀이
```python3

COLUMN_MAP = {
    "a": 1,
    "b": 2,
    "c": 3,
    "d": 4,
    "e": 5,
    "f": 6,
    "g": 7,
    "h": 8,
}
def solution(position):
    x, y = int(COLUMN_MAP.get(position[0])), int(position[1])

    count = 0
    directions = [
        (1, -2), # 1시 방향
        (2, -1), # 2시 방향
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
    ]
    for (move_x, move_y) in directions:
        x_new = x + move_x
        y_new = y +move_y

        if (1 <= x_new) and (x_new <= 8) and ( 1 <= y_new) and ( y_new <= 8):
            count += 1

    return count

if __name__ == "__main__":
    assert solution("a1") == 2
```

&nbsp;

## [문제 상황] 문자열 재정렬
알파벳 대문자와 숫자(0~9)로만 구성된 문자열이 입력으로 주어집니다.
이때, 모든 알파벳을 오름차순으로 정렬하여 이어서 출력한 뒤에, 그 뒤에 모든 숫자를 더한 값을 이어서 출력합니다.


### 풀이
``` python3

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
```