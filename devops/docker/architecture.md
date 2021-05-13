# Docker Architecture Overview

도커는 내부적으로 클라이언트-서버 구조를 사용한다. 도커는 Docker daemon이라 불리는 데몬몬 실행하여 컨테이너와 관련된 모든 요청(빌드, 실행 등)을 처리한다. 도커의 기본 CLI 클라이언트는 UNIX 소켓(default) 혹은 네트워크 인터페이스를 통해 도커 데몬으로 REST API 요청을 보내게 된다.

[https://docs.docker.com/engine/images/architecture.svg](https://docs.docker.com/get-started/overview/)

&nbsp;

## Docker Daemon

도커 데몬(`dockerd`)는 Docker API 요청을 수신하고 이미지, 컨테이너, 네트워크, 볼륨 등의 도커 오브젝트를 관리한다. 

&nbsp;
## Docker Client

도커 클라이언트(`docker`)는 도커 서버와 통신하기 위한 가장 주요한 기능을 수행한다. 예를 들어, docker run 명령어를 실행하면 도커 클라이언트는 해당 명령어를 REST API Call으로 변환하여 도커 데몬(`dockerd`)로 전송한다. 

&nbsp;
## Docker Registries

도커 레지스트리는 도커 이미지를 저장하는 저장소 역할을 한다. `Docker Hub`는 퍼블릭 도커 레지스트리로, 도커는 레지스트리 기본 값으로 Docker Hub를 사용한다. 사용자는 프라이빗 레지스트리를 구축하여 사용할 수 있다. 사용자가 `docker pull` 혹은 `docker run`과 같은 명령어를 실행하면, 도커는 사용자가 요청한 이미지를 도커 레지스트리에서 찾아서 가져온다. 사용자가 `docker push` 명령어를 실행하면 도커는 이미지를 레지스트리에 저장한다.

&nbsp;

## Docker Objects

도커에서 이뤄지는 명령어는 대부분 이미지, 컨테이너, 네트워크, 볼륨과 같은 도커 오브젝트를 관리하는 역할을 수행한다. 도커 오브젝트는 도커의 컨테이너 기술의 핵심 기능들을 담당하고 있으며, 도커는 다양한 도커 오브젝트를 지원한다.

### Images

이미지는 read-only 템플릿으로 도커 컨테이너를 생성하기 위해 필요한 절차를 기록한 것이다. 도커의 이미지는 유니온 마운트 기반의 파일 시스템에서 레이어(layer)를 중첩하여 쌓는 방식으로 생성된다. 대개의 경우, 이미지는 기본 이미본 위에 추가로 명령어를 쌓는 방식으로 생성될 수 있다. 예를 들어, 사용자는 `ubuntu` 이미지 위에 다양한 어플리케이션을 얹어 새로운 이미지를 빌드할 수 있다.

사용자는 자신이 빌드한 이미지를 `docker push` 명령어를 활용해 도커 레지스트리로 업로드 할 수 있다. 이미지를 직접 생성하기 위해서는 `Dockerfile`에 이미지 생성을 위한 명령어를 추가하고 `docker build` 명령어를 활용할 수 있다. 도커의 이미지는 레이어(layer)를 중첩하는 방식으로 생성되기 때문에, 한번 빌드된 이미지의 중간 레이어는 캐싱되어 이후 다른 이미지를 빌드할 때 재활용 할 수 있다.

### Containers

컨테이너는 이미지를 실행한 결과로 생성되는 인스턴스를 의미한다. 사용자는 도커 클라이언트의 명령어를 호출함으로써, 컨테이너를 관리할 수 있다. 도커의 컨테이너는 네임스페이스와 컨트롤 그룹을 호스트 환경에서 격리시킨 하나의 프로세스로 컨테이너는 다양한 네트워크 네임스페이스와 연결될 수 있고, 새로운 볼륨을 해당 컨테이너 네임스페이스에 마운트 하는 것도 가능하다. 

### Example `docker run` command

아래의 명령어는 `ubuntu` 컨테이너를 생성하고, `/bin/bash` 프로그램을 실행하여 현재 커널 세션과 연결한다.

```bash
$ docker run -it ubuntu /bin/bash
```

위 명령어를 실행하면, 도커는 다음의 작업을 수행한다. 
1. 만약 로컬 환경에 `ubuntu` 이미지가 존재하지 않는 경우, 도커 데몬은 해당 이미지를 도커 레지스트리에서 찾아서 가져오는 작업(`docker pull ubuntu`)을 수행한다.

2. `ubuntu` 이미지를 기반으로 새로운 컨테이너를 생성(`docker container create`)한다. 


### References

[Docker 공식 문서](https://docs.docker.com/get-started/overview/)