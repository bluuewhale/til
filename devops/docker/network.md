# Docker Network: Under the Hood

> 이 글은 44bits 블로그에 Daegwon Nacyot Kim님께서 정리해주신 리눅스/도커 네트워크 관련 글을 정리한 내용입니다. 원문에 대한 링크는 글 하단의 Refernce에서 확인하실 수 있습니다.

&nbsp;

## Docker Container Magic

리눅스 시스템에서 네트워크(네트워크 인터페이스, 라우팅 테이블)는 오직 하나만 존재한다. 그런데 도커 컨테이너 호스트에서 사용중인 포트를 중복으로 사용할 수 있다. 도커는 어떻게 이를 가능하게 했을까?

&nbsp;

## What Exactly is a Docker Container?
도커 컨테이너는 하드웨어 가상화 없이 실행되는 하나의 프로세스이다. 그럼에도 불구하고 도커의 컨테이너는 마치 다른 네트워크 환경에서 작동하는 것처럼 보인다. 이는 도커가 리눅스 네임스페이스를 활용하여 각각의 컨테이너에 대한 네트워크를 격리시켰기 때문이다. 도커 컨테이너에 대한 설명은 "Daegwon Nacyot Kim"님이 작성하신 [도커 컨테이너는 가상머신인가요? 프로세스인가요?](https://www.44bits.io/ko/post/is-docker-container-a-virtual-machine-or-a-process)에서 확인하실 수 있습니다.

&nbsp;

## Linux Namespace & Docker Network
리눅스에는 다양한 네임스페이스(PID, Network, UID, Mount, UTS, IPC)가 존재한다. 이 중에서도 도커는 리눅스 시스템의 네트워크 네임스페이스와 UTS 네임스페이스를 활용하여 컨테이너의 네트워크를 격리시킨다. 

네트워크 네임스페이스는 네트워크 리소스(네트워크 디바이스, IP, Port, 라우팅 테이블)를 네임스페이스 단위로 격리하여 프로세스를 독립적으로 운영할 수 있도록 한다. UTS 네임스페이스는 네임스페이스 단위로 호스트명과 도메인 명을 격리하는 것을 가능케한다. 도커는 컨테이너마다 별도의 네트워크/UTS 네임스페이스를 생성함으로써, 컨테이너의 네트워크를 호스트 네트워크에서 논리적으로 분리한다.

&nbsp;

## UTS Namespace Isolation
이 글에서는 네트워크 네임스페이스 관한 내용을 좀 더 중점적으로 다루었습니다. 리눅스 시스템에서 UTS 네임스페이스 분리하는 방법에 대한 내용은 [이 글](https://www.44bits.io/ko/post/container-network-1-uts-namespace)을 참조하시면 좋을 것 같습니다.

&nbsp;

## Network Namespace Isolation
리눅스 시스템에서는 네트워크 네임스페이스를 분리하는 명령어(unshare, ip netns add HOST_NAME)를 제공한다. 새롭게 생성된 네트워크 네임스페이스(ns1)는 호스트 네임스페이스에서 분리되며, 만들어진 직후에는 네트워크 설정이 없어 연결이 불가능하다. 새로운 네트워크 네임스페이스와 통신하기 위해서는 먼저, 두 네임스페이스를 연결하는 veth(virtual ethernet)쌍을 만들어야 한다. veth는 한 쌍으로 생성되며 두 네트워크를 연결하는 파이프 역할을 수행한다. veth 생성과 관련된 자세한 내용은 [veth: 리눅스 가상 이더넷 인터페이스란?](https://www.44bits.io/ko/keyword/veth#veth%EB%B2%84%EC%B6%94%EC%96%BC-%EC%9D%B4%EB%8D%94%EB%84%B7-%EC%9D%B8%ED%84%B0%ED%8E%98%EC%9D%B4%EC%8A%A4%EB%8A%94)에 자세히 설명되어 있습니다.

만들어진 veth 쌍은 기본적으로 init 프로세스(PID 1)의 네임스페이스를 공유한다. 따라서, 아직은 호스트 네트워크에서 분리된 네트워크(ns1)에 접근할 수 없다. 이를 위해서는 만들어진 veth를 호스트 네트워크와 ns1 네트워크에 각각 위치시킨 후, IP를 부여하고 활성화 해주어야 한다. 위 과정은 [ip로 직접 만들어보는 네트워크 네임스페이스와 브리지 네트워크](https://www.44bits.io/ko/post/container-network-2-ip-command-and-network-namespace)에 상세히 설명되어 있습니다.

이러한 과정을 마치면 리눅스 환경에서 분리된 네트워크 네임스페이스 간의 통신이 가능해진다. 

&nbsp;

## Sumamry
도커는 리눅스 시스템의 UTS/네트워크 네임스페이스를 활용하여 각 컨테이너의 네트워크를 격리한다. 도커는 기본값으로 브릿지(L2) 네트워크를 사용한다는 점에서 조금 차이가 있지만, 기본적으로는 위 과정을 거쳐 컨테이너의 네트워크를 분리하고 각각의 컨테이너를 실행시킨다. 도커에는 브릿지 이외에도 다양한 네트워크 드라이브를 제공한다. 


&nbsp;

---
### References
[Network Namespace](https://www.joinc.co.kr/w/man/12/NetworkNamespace)
[UTS 네임스페이스를 사용한 호스트네임 격리](https://www.44bits.io/ko/post/container-network-1-uts-namespace)
[ip로 직접 만들어보는 네트워크 네임스페이스와 브리지 네트워크](https://www.44bits.io/ko/post/container-network-2-ip-command-and-network-namespace)
[Linux - Namespace 란?](https://galid1.tistory.com/442)
[veth: 리눅스 가상 이더넷 인터페이스란?](https://www.44bits.io/ko/keyword/veth#veth%EB%B2%84%EC%B6%94%EC%96%BC-%EC%9D%B4%EB%8D%94%EB%84%B7-%EC%9D%B8%ED%84%B0%ED%8E%98%EC%9D%B4%EC%8A%A4%EB%8A%94)