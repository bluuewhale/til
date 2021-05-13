# Docker: UFS(Union File System)

앞선 포스트에서 도커는 이미지를 레이어 단위로 관리하고 각 레이어를 재사용함으로써 이미지를 효율적으로 빌드한다는 내용을 다루었다. 이번 포스트에서는 도커가 어떻게 이미지를 레이어 단위로 관리하는지에 대해 다루어 보도록 하자.

## UFS
UFS는 Union File System의 약자이다. UFS는 여러 개의 파일 시스템을 하나의 파일 시스템에 마운트 하는 기능이다. 여러 파일 시스템을 하나로 합치다보면 중복되는 파일이 있기 마련인데, UFS에서는 이 경우 나중에 마운트된 파일로 덮어쓴다(overlay).도커 이미지에서 레이어는 개념적으로 각각의 파일 시스템을 겹쳐 놓은 형태와 유사하다.

UFS를 그림으로 표현하면 다음과 같은 형태이다.

![](https://miro.medium.com/max/700/1*hZgRPWerZVbaGT8jJiJZVQ.jpeg)

*<center> image by Ridwan Shariffdeen from Medium</center>*

&nbsp;

## UFS in Docker

도커의 컨테이너는 UFS를 기반으로 동작한다. 도커의 이미지와 컨테이너의 관계는 일반적으로 바이너리 프로그램과 프로세스의 관계와 유사하다. 하나의 바이너리 프로그램에서 여러 개의 프로세스가 독립적으로 실행될 수 있듯이, 하나의 이미지로 여러 컨테이너를 독립적으로 실행할 수 있다.

그런데, 하나의 이미지에서 생성된 컨테이너들이 동일한 파일 시스템을 공유하고 있다면 어떻게 컨테이너가 독립적으로 실행될 수 있을까? 해답은 UFS의 CoW(Copy on Write) 전략과 관련이 있다.

UFS에서는 기존 레이어(하위 레이어) 위에 새로운 레이어(상위 레이어)가 쌓일 경우, 하위 레이어는 읽기 전용 상태가 된다. 또한, 상위 레이어에서 하위 레이어에 쓰기 작업을 수행할 경우, 하위 레이어를 복사하여 사용(CoW)하기 때문에 상위 레이어에서는 하위 레이어에 아무런 영향을 주지 않는다.

도커에서는 레이어가 크게 컨테이너 레이어(상위)와 이미지 레이어(하위)로 구분된다. UFS 특성에 따라서 컨테이너가 파일 시스템에 쓰기 작업을 수행할 경우, 실질적으로는 하위 레이어의 복사본에 해당 작업을 수행하기 때문에 서로 다른 컨테이너가 하위 레이어를 공유하고 있어도 서로 독립적인 파일 시스템 운용이 가능해진다.

이를 그림으로 표현한 결과물은 다음과 같다. 
![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FJFZsx%2Fbtqt1jKHoQh%2FZMYu9kuQg1TVtNzbMcZg3K%2Fimg.jpg)

*<center> Images from Devaom's Tech Blog </center>*

도커에서 관리되는 모든 레이어와 관련된 정보는 호스트의 파일 시스템 내의 /var/lib/docker 폴더에 저장된다. 이 영역을 Docker area 혹은 Backing Filesystem 이라고 부른다.

### References

[[Docker] 컨테이너의 구조](https://devaom.tistory.com/5)
[도커 컨테이너 까보기(2) – Container Size, UFS](http://cloudrain21.com/examination-of-docker-containersize-ufs)