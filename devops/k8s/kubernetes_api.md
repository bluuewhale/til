# The Kubernetes API

쿠버네티스 API 서버는 쿠버네티스 Control Plane의 핵심으로, 클러스터에 속한 노드 혹은 외부의 사용자로부터 HTTP API 요청을 받아 클러스터의 상태를 관리하는 기능을 수행한다. 

&nbsp;

## Primary Roles

쿠버네티스의 API 서버는 클러스터에서 다음의 기능을 수행한다. 이번 포스트에서는 API 서버로서 어떻게 동작하는지에 초점을 맞춰 글을 작성하였다.

- API 서버 
   
  API 서버로서, 클러스터에 속한 노드와 외부 사용자에게 `Pod` 이나 `Service`와 같은 쿠버네티스 오브젝트를 제어할 수 있는 엔드포인트를 제공한다.  쿠버네티스의 API는 기본적으로 API 서버는 기본적으로 `JSON` 포맷의 HTTP 요청(POST, PUT, DELETE, GET)을 받아서 API 오브젝트를 관리하며, intra-cluster 커뮤니케이션을 위해 Protocol Buffer를 사용하기도 한다. 

- 프록시 
  
  쿠버네티스 컴포넌트의 프록시 역할을 수행한다. ex) `Kubernetes UI`

- 슬러스터 상태 관리
  
   쿠버네티스 클러스터의 상태를 `etcd`에 저장하여 안전하게 관리한다. 쿠버네티스 API 서버는 쿠버네티스 오브젝트의 상태를 직렬화하여 `etcd`에 저장한다. `etcd`는 raft 알고리즘 기반의 분산형 key-value 저장소로 클러스터의 상태와 관련된 정보들이 저장된다. 

&nbsp;

![](https://www.programmersought.com/images/88/7dd3a21f02aaf7aa54f89bb07c2512f8.png)

&nbsp;

## API Group Structure and Versioning
쿠버네티스 HTTP API는 다음과 같은 계층구조를 가지며, 크게 3가지 그룹으로 구성되어있다.

![](https://www.programmersought.com/images/684/63e897a9190472d71907022801c92ad4.png)

### Core Group
`/api/v1` 경로 아래 위치하며, 쿠버네티스의 핵심 리소스들 관리를 위한 엔드포인트를 제공한다.

### Named Group
`/apis/NAME/VERSION` 경로에 위치하며, Core Group에 속하지 않은 다른 리소스들와 관련된 엔드포인트를 제공한다.

### System-Wide
`helathz` 혹은 `metrics`와 같이 시스템과 관련된 정보를 제공하는 API 그룹이다.

&nbsp; 

쿠버네티스는 확장성을 위해, `/apis/batch/v1`, `apis/batch/v2apha1/v1`과 같이 다양한 API 버전과 이에 따른 엔드포인트를 제공한다. 프록시를 거쳐 직접 API 서버에 요청을 보내면 `apis/batch/v2alpha1` 버전이 더 많은 기능을 제공하는 것을 확인할 수 있다.

&nbsp; 

```bash
$ curl http://127.0.0.1:8080/apis/batch/v1
{
    "kind": "APIResourceList",
    "apiVersion": "v1",
    "groupVersion": "batch/v1",
    "resources": [
    {
        "name": "jobs",
        "namespaced": true,
        "kind": "Job"
    },
    {
        "name": "jobs/status",
        "namespaced": true,
        "kind": "Job"
    }
]
}
```

```bash
$ curl http://127.0.0.1:8080/apis/batch/v2alpha1
{
    "kind": "APIResourceList",
    "apiVersion": "v1",
    "groupVersion": "batch/v1",
    "resources": [
    {
        "name": "cronjobs",
        "namespaced": true,
        "kind": "CronJob"
    },
    {
        "name": "cronjobs/status",
        "namespaced": true,
        "kind": "CronJob"
    },
    {
        "name": "jobs",
        "namespaced": true,
        "kind": "Job"
    },
    {
        "name": "jobs/status",
        "namespaced": true,
        "kind": "Job"
    },
    {
        "name": "scheduledjobs",
        "namespaced": true,
        "kind": "ScheduledJob"
    },
    {
        "name": "scheduledjobs/status",
        "namespaced": true,
        "kind": "ScheduledJob"
    }
]
}
```

&nbsp; 

> Note: 쿠버네티스의 버전은 크게 3가지 종류가 존재하며, 각각은 버전의 성숙도를 나타낸다.
> 
> - **Alpha phase**:  alpha phase에서는 실험적인 기능들과 관련되어 있다.
> 
> - **Beta phase**: alpha phase를 거쳐, 테스트 환경에서 기본적인 기능 검증을 마친 단계이다.
>
> - **Stable phase**: 실제 프로덕션 환경에서 사용할 수 있도록 안정성이 검증된 단계이다.


&nbsp;

쿠버네티스에서는 버저닝이 필드나 리소스 레벨이 아닌 API 레벨에서 이뤄지기 때문에, 각각의 API 요청이 보다 직관적이고 우수한 확장성을 갖는다. API 리소스에 대한 HTTP 요청은 아래와 같이 API 그룹, 버전, 리소스 타입, 네임스페이스, 리소스 명으로 구분되어진다. 

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FcGDuEP%2FbtqDSjPXXM9%2FTtKQH37kpj9cjl9k9yEJM0%2Fimg.png)

*<center> images from [한우형의 기술 블로그](https://woohhan.tistory.com/44)</center>*

특징적인 점은 동일한 리소스에 대해서는 서로 다른 버전으로 HTTP 요청을 보내도 내부적으로 이를 변환하여 적절히 처리한다는 것이다. 예를 들어, `v1beta1` 버전으로 생성된 리소스에 대해 `v1` 버전으로 쿠버네티스 API 서버에 요청을 보내면, API 서버는 먼저 해당 요청을 내부 버전으로 변환한 후, 이를 다시 `v1beta` 버전으로 변환하여 요청을 처리한다. 

이러한 내부 변환 과정 덕분에 사용자는 버전 변화로 인한 컨텍스트 변화로부터 상대적으로 자유롭다.

&nbsp;

---
### References
[In-depth analysis of the Kubernetes API Server trilogy - part 1](https://www.programmersought.com/article/47541020995/)
[Kubernetes 공식 문서: The Kubernetes API](https://kubernetes.io/docs/concepts/overview/kubernetes-api/)
[Kubernetes 코드 분석: kube-apiserver](https://woohhan.tistory.com/44)

