# Kubernetes Network Explained

### Pods

Pod이란 동일한 host에서 network stack, volume등을 공유하는 컨테이너의 집합으로, kubernetes application을 구성하는 기본 단위이다. 여기서 network stack을 공유한다는 말은, localhost를 통해 컨테이너가 서로 통신할 수 있다는 것을 의미한다. 아래의 그림은 컨테이너가 네트워크 스택을 공유하는 방법을 표현한 것이다.

[](https://miro.medium.com/max/700/1*0Xo-WpbTTGKZhJt7TvFLZQ.png)

<center>image from medium(Mark Betz)</center>

---
### References
![Understanding kubernetes networking: pods](https://medium.com/google-cloud/understanding-kubernetes-networking-pods-7117dd28727)
![[번역] 쿠버네티스 네트워킹 이해하기#1: Pods](https://coffeewhale.com/k8s/network/2019/04/19/k8s-network-01/)