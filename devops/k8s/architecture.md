
# Kubernetes Architecture Explained

## Kubernetes Components

## [1] Control Plane

Control plane은 쿠버네티스 클러스터의 핵심으로 클러스터의 상태와 설정 값을 저장/관리한다. Control plane은 하나 이상의 master node에 복제되어 관리된다.

&nbsp;

<center><img src="https://platform9.com/wp-content/uploads/2019/05/Kubernetes-control-plane-taxonomy-480x724.jpg" alt="drawing" style="width:300px;"/></center>

*<center>images from Platform9</center>*

&nbsp;


### kube-apiserver

Kubernetes API는 쿠버네티스 클러스터의 frontend 기능을 담당한다. API 서버는 REST call 형태로 클러스터 내부와 외부에서 요청을 받아 이를 처리한다. Client는 API 서버를 통해 요청을 전송하고, node와 pod에 접속할 수 있다.

### kube-scheduler

Kubernetes-scheduler는 요청(node selector, anti-affinity rules 등)을 분석하여, 새롭게 생성되는 Pod에 적절한 자원(CPU, memory, node 등)을 배분하는 역할을 수행한다.

### kube-controller-manager

kube-controller-manager는 클러스터 운영에 필요한 controller들을 모아둔 하나의 binary 프로그램이다. kube-controller-manager에는 다음의 controller들이 포함되어 있다.

- Node controller: 노드의 상태를 관리한다.
- Job controller: Job specification에 맞춰, 해당 작업이 실행될 수 있는 Pod을 생성하는 역할을 한다.
- Endpoint controller: Endpoint objects(Services, Pods)를 생성한다.
- Service Account & Token Controller: default account를 생성하고, namespace 별로, access token을 관리한다.

### cloud-controller-manager

Cloud Controller Manager는 개별 Cloud Provider API와 통합하여, 현재 kubernetes 클러스터 상태에 맞춰, Cloud 환경에서 VM 인스턴스, Storage, 네트워크, 라우팅, 로드 밸런서 등을 생성하는 역할을 담당한다.

Node controller: 지속적인 헬스체크로 노드의 상태를 관리한다.
Route controller: Cloud 환경에 라우팅 정보를 설정한다.
Service controller: Cloud 환경에 로드 밸런서를 생성/관리/제거 하는 기능을 담당한다.


### etcd

etcd는 raft 알고리즘 기반의 분산형 key-value 저장소로, 클러스터의 상태와 관련된 정보들이 저장된다. 

&nbsp;

## [2] Cluster Nodes

kubernetes 클러스터는 한 개 이상의 클러스터 노드로 구성된다. 클러스터 노드는 마스터 노드에 의해 관리되며 실제로 container를 실행하는 기능을 담당한다. 모든 클러스터 노드에는 kubelet과 kube-proxy 서비스가 실행된다. 

&nbsp;

<center><img src="https://platform9.com/wp-content/uploads/2019/05/Kubernetes-node-taxonomy-480x581.jpg" alt="drawing" style="width:300px;"/></center>

*<center>images from Platform9</center>*

&nbsp;

### kubelet
kubelet은 지속적으로 desired state와 관련된 정보(ex, API call from control plane)를 받아 container를 실행하고, 해당 상태를 유지되도록 관리한다.

### kube-proxy
kube-proxy는 kubernetes 클러스터 내부의 네트워크를 담당하는 proxy이다. 

### Container runtime 

Container runtime은 실제로 컨테이너가 실행되는 환경을 제공한다. Kubernetes에서 실행되는 container runtime은 Kubernetes CRI(Container Runtime Interface)를 구현하여야 한다. 대표적인 container runtime으로 docker, containerd 등이 있다. 

&nbsp;

--- 
### References

[kubernetes 공식문서](https://kubernetes.io/docs/concepts/overview/components/)

[RedHat: Introduction to Kubernetes architecture](https://www.redhat.com/en/topics/containers/kubernetes-architecture)

[Platform9: Kubernetes Concepts and Architecture](https://platform9.com/blog/kubernetes-enterprise-chapter-2-kubernetes-architecture-concepts/#Kubernetes-Control-Plane)