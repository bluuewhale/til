""" 행동(Behavior) 패턴
https://victorydntmd.tistory.com/294

현재 state에 따라 행동을 다르게 해야하는 경우,
Client가 state를 확인하고 행동을 직접 선택할 필요 없이, state를 클래스로 정의함으로써 인터페이스를 통일


스테이트패턴과 스트래티지패턴은 동일한 구조를 가지지만, 어떤 목적을 위해서 사용되는가를 생각해보면 답이 나옵니다. 
(https://github.com/KWSStudy/Refactoring/issues/2)

스트레티지 - 구성을 이용하여 상속을 대체
스테이트 - 구성을 이용하여 코드내의 조건문들을 대체
"""
