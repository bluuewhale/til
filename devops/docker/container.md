> 이 글은 44Bits에 Daegwon Kim님이 작성해주신 글 [컨테이너란? 리눅스의 프로세스 격리 기능](https://www.44bits.io/ko/keyword/linux-container)을 요약 정리한 내용입니다.

&nbsp; 

# Docker Container: Under the Hood
컨테이너라는 개념은 도커가 생기기 이전에 리눅스 시스템에 이미 존재하던 개념입니다. 도커는 리눅스 컨테이너의 종류 중 하나인 어플리케이션 컨테이너를 효율적으로 실행할 수 있는 런타임을 제공합니다. 이 글에서는 도커 컨테이너의 근간을 이루는 리눅스 컨테이너에 대해 다루도록 하겠습니다.

&nbsp; 

## Linux Container
리눅스의 컨테이너는 하드웨어 가상화(Virtual Machine) 없이 커널을 공유하고 프로세스를 서로 다른 네임스페이스(ex, Network, UTS), 컨트롤 그룹, 루트 디렉토리(chroot) 등을 격리하여 실행하는 기술을 의미합니다. 사용자 입장에서 컨테이너는 프로세스가 독립적인 가상 머신 환경에서 실행되는 것처럼 보이지만, 실제로는 호스트 머신에서 실행되고, 리눅스의 격리기술의 총집합으로 탄생한 하나의 프로세스입니다.

&nbsp; 

## Docker vs Linux Container
lxc(Linux Container)는 네임스페이스와 컨트롤 그룹을 분리하여 각각의 프로세스를 샌드박싱하여 독립된 런타임 환경을 제공하는 기술을 의미합니다. 도커는 low-level의 lxc 관련 기술(현재는 containerd와 runc를 사용)을 기반으로 컨테이너를 효율적으로 생성/실행/관리 할 수 있는 high-level 기능을 제공합니다. 

&nbsp; 

### Portable Deployment 
도커는 하나의 어플리케이션이 서로 다른 머신 환경에서도 동일하게 실행될 수 있음을 보장합니다. 도커는 lxc 기술을 기반으로 컨테이너를 생성/관리하며, 컨테이너 환경, 즉 컨테이너 내부에서 실행되는 프로세스를 호스트로부터 격리(sandboxing)하여 각각의 컨테이너에 독립된 런타임 환경을 제공합니다.

&nbsp; 

### Commponent re-use & Sharing
도커는 유니온 마운트를 지원하는 파일 시스템(ex, OverlayFS)을 기반으로 컨테이너의 파일 시스템을 효율적으로 생성함으로써 효율적인 이미지 빌드 기능을 제공합니다. 도커는 사용자가 빌드된 이미지를 서로 공유할 수 있는 레지스트리(Docker Hub)를 제공하여 사용자가 원하는 이미지를 쉽고 빠르게 찾을 수 있습니다.

&nbsp; 

## Summary
- 도커 컨테이너의 근간에는 리눅스 컨테이너 기술이 존재한다.
- 도커는 리눅스 컨테이너의 라이프사이클(ex, 생성, 실행, 관리, 제거)을 효율적으로 관리할 수 있는 다양한 기능을 제공한다.

&nbsp; 

---

### References
[컨테이너란? 리눅스의 프로세스 격리 기능](https://www.44bits.io/ko/keyword/linux-container)
[What does Docker add to lxc-tools (the userspace LXC tools)?](https://stackoverflow.com/questions/17989306/what-does-docker-add-to-lxc-tools-the-userspace-lxc-tools)