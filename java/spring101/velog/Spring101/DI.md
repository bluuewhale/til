> 이 글은 백기선님의 Inflearn 강좌 [예제로 배우는 스프링 입문(개정판)](https://www.inflearn.com/course/spring_revised_edition/dashboard)을 정리한 내용입니다.

# Dependency Injection
DI는 `의존성 주입`을 의미한다. 의존성 주입이란 하나의 객체 A가 다른 객체 B를 사용(의존)해야 할 때, A의 코드 내부에서 B를 만드는 것이 아니라, 외부(IoC 컨테이너)에서 B를 만들고 생성자 혹은 `setter` 메서드 등을 활용해 A 내부로 주입하는 것을 의미한다. 이러한 방식은 모듈 간의 결합을 낮춰 수정이 발생하였을 때 코드 변경지점을 최소화 할 수 있다는 장점이 있다.

&nbsp;

## Inversion of Control
IoC는 제어권 역전으로, 일반적으로 개발자가 프로그램의 흐름을 제어하는 것이 아니라, 프레임워크가 프로그램의 흐름을 주도하는 것을 의미한다. 일반적인 프로그램의 흐름은 다음과 같다.

`객체 생성(A)` -> `내부에서 의존성 객체 생성(B)` -> `의존성 객체(B)의 메서드를 호출`
```java
class OwnerController {
    private OwnerRepository repository = new OwnerRepository();
}
``` 

&nbsp;

그러나, IoC가 적용된 경우에는 프로그램의 흐름이 다음과 같은 형태로 진행된다. 스프링에서는 모든 의존성 객체(`Bean`)를 싱글톤 Scope에서 IoC 컨테이너에서 직접 관리하며, 필요한 곳에 주입해주는 역할을 한다.

`객체 생성(A, B)` -> `의존성 객체 주입(B)` -> `의존성 객체(B)의 메서드 호출`

```java
class OwnerController {
    private final OwnerRepository repository;

    public OwnerController(OwnerRepository repository) {
        this.repository = repository;
    }
}
```
&nbsp;

## Bean

Bean은 IoC 컨트롤러가 생성/관리하여 자동으로 DI를 제공하는 객체를 의미한다. 클래스를 Bean으로 등록하기 위한 방법은 크게 2가지가 있다.

#### Component Scanning
  
`@SpringBootApplication` 어노테이션 안에는 `@ComponentScan` 어노테이션이 포함되어 있다. `@ComponentScan` 어노테이션은 해당 어노테이션이 붙어있는 위치에서 하위 패키지를 탐색하며 `@Component` 어노테이션이 붙어있는 클래스를 찾아 `Bean`으로 등록하라고 지시하는 역할을 한다. `@Component` 어노테이션 포함된 어노테이션은 대표적으로 `@Controller`, `@Repository`, `@Service`, `@Configuration`등이 있다.

&nbsp;

#### 직접 등록
  
xml 혹은 자바 설정파일을 사용하여 직접 Bean을 등록할 수 있다.

```java
@Configuration
public class SampleConfig {

    @Bean
    public SampleController sampleController() {
        return new SampleController();
    }
}
```

&nbsp;

## @Autowired
`@Autowired`는 IoC 컨트롤러에게 의존성을 주입해달라고 지시하는 어노테이션이다. `@Autowired`를 사용하는 방법은 크게 3가지가 있다.

#### 생성자

필수적으로 사용되는 객체를 생성 시점에 강제할 수 있기 때문에, 생성자를 사용하여 의존성을 주입하는 방법은 **Spring 공식 레퍼런스에서 권장하는 방법**이다. 단, 순환참조가 발생하는 경우는 예외적으로 아래의 방법을 활용하여 해결할 수 있다.

Spring 4.3버전 부터는 Bean으로 등록된 객체에 유일한 생성자에 매개변수들이 Bean으로 등록되어 있다면, IoC 컨트롤러에서 `@Autowired` 어노테이션을 생략해도 된다.

```java
@Controller
class OwnerController {
    
    private final OwnerRepository owners;
    
    @Autowired
    public OwnerController(OwnerRepository owners) {
        this.owners = owners;
    }
}
```

#### 필드

의존성 주입을 원하는 필드에 `@Autowired`를 추가하는 방식으로도 사용할 수 있다.

```java
@Controller
class OwnerController {
    
    @Autowired
    private OwnerRepository owners;
    
    ...
}
```

#### Setter

```java
@Controller
class OwnerController {
    
    private OwnerRepository owners;

    @Autowired
    public void setOwners(OwnerRepository owners) {
        this.owners = owners;
    }
}
```