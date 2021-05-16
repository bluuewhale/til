# Min cost climbing staris

# 0번 혹은 1번 인덱스에서 시작할 수 있다.
# 한번에 1칸 혹은 2칸 움직일 수 있다.
# 비용 배열의 마지막 인덱스를 벗어나면 계단을 모두 오른 것으로 간주한다.
# 계단을 모두 오를 때까지 필요한 최소한의 비용을 계산하라

# 해답
# 뒤에서 부터 이터레이션을 시작하여, 현재 위치부터 마지막까지 갈 때 필요한 최소한의 
# 비용을 캐싱하여 문제를 푼다. 유사한 문제로 미로에서 최적의 경로 탐색 문제가 있다.
# 최적 경로 탐색은 출구에서 시작해서 입구로 가는 방식으로 풀어낼 수 있다.

from collections import defaultdict

def solution(costs):
    
    cost_map = dict(enumerate(costs))
    n = len(costs)
    for i in reversed(list(range(n))):
        if i == 1:
            break

        cost_map[i-2] += min(cost_map[i-1], cost_map[i])
    return min(cost_map[0], cost_map[1])

def test1():
    costs = [10, 15, 20]
    assert solution(costs) == 15

def test2():
    costs = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
    assert solution(costs) == 6

if __name__ == "__main__":
    test1()
    test2()