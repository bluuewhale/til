> 이 글은 백기선님의 Inflearn 강좌 [예제로 배우는 스프링 입문(개정판)](https://www.inflearn.com/course/spring_revised_edition/dashboard)을 정리한 내용입니다.

# Aspect Oriented Programming

하나의 메서드를 실행할 때, 부수적으로 함께 실행되어야 하는 메서드들이 존재할 수 있다. 예를 들어, 인증, 로깅, 트랜젝션, 캐싱과 같은 부가기능들은 비지니스 로직 전반에 걸쳐 빈번하게 사용된다. 이러한 코드를 핵심 비지니스 로직과 함께 작성하여 사용할 경우 불필요한 중복이 발생하고, 유지보수가 번거로워지는 단점이 있다. 이러한 문제 하나의 영역에서 처리하는 방식으로 우아하게 해결하여, 본래의 메서드는 본인의 역할에 집중할 수 있도록 하는 것이 AOP의 핵심이다. Spring에서는 프록시 패턴을 활용하여 AOP를 구현한다.

&nbsp;

## AOP의 구현방법

#### 컴파일 시
`*.java` 파일을 컴파일하여 `*.class` 파일을 생성하는 시점에 자동적으로 AOP 기능을 추가하는 기능을 의미한다. 대표적으로 `AspectJ`가 있다.

#### 바이트코드 조작
바이트 코드 조작 방식은 `*.class` 클래스 로더가 클래스 파일을 읽어 메모리에 적재하기 직전에 AOP 관련 기능을 추가하는 방법론이다.

#### 프록시 패턴 (`Spring AOP`)
프록시 패턴 방식은 일반적으로 널리 활용되는 디자인 패턴의 하나로, `데코레이터` 혹은 `프록시` 객체를 사용하여 메서드의 호출부를 감싸는 방식으로 AOP 기능을 제공한다. 아래의 예제는 가장 단순한 형태의 프록시 패턴이 적용되는 사례를 구현한 것으로 실제 스프링에서 구현한 AOP 코드와는 차이가 있다. 

> Spring 101에서는 스프링 AOP의 직접적인 구현과 관련된 상세한 내용을 자세히 다루지 못 하였습니다.


```java
// Worker.java
public interface Worker {
    void work();
}

public class HardWorker implements Worker {
    public void work() {
        System.out.println("Work hard!");
    }
}

public class LazyWorker implements Worker {
    public void work() {
        System.out.println("I'm lazy");
    }
}

// WorkerTimer는 Worker 인터페이스를 구현한 클래스의 프록시 클래스러, 핵심 메서드를 실행과 더불어 부가적인 기능(ex, 캐싱, 로깅)을 함께 수행한다.
public class WorkerTimer implements Worker {
    Worker worker;

    public WorkerTimer(Worker worker) {
        this.worker = worker;
    }

    public void work() {
        StopWatch stopWatch = new StopWatch();
        stopWatch.start();

        this.worker.work()

        stopWatch.stop();
        System.out.println(stopWatch.prettyPrint());
    }
}

public class WorkSpace {
    public void work(Worker worker) {
        worker.work();
    }
}
```


## AOP in Spring
스프링에서 제공하는 AOP 관련 기능은 매우 다양하다. 이 글에서는 가장 단순한 형태로 로깅을 담당하는 AOP를 구현하였다.
Spring에서 AOP는 크게 `Annotation 정의` -> `Aspect 정의` -> `Annotation 사용` 과정을 거쳐 구현된다. 



```java
// LogExecutionTime.java

@Target(ElementType.METHOD)
@Retention(RetentionPolicy.RUNTIME)
public @interface LogExecutionTime { 
}
```

```java
// LogAspect.java

@Component
@Aspect
public class LogAspect {
    Logger logger = LoggerFactory.getLogger(LogAspect.class);

    @Around("@annotation(LogExecutionTime)")
    public Object logExecutionTIme(ProceedingJoinPoint joinPoint) throws Throwable {
        StopWatch stopWatch = new StopWatch();
        stopWatch.start();

        Object result = joinPoint.proceed();

        stopWatch.stop();
        System.out.println(stopWatch.prettyPrint());

        return result;
    }
}
```

```java
// OwnerController.java

@Controller
public class OwnerController {
    ...

    @GetMapping("/owners/find")
    @LogExecutionTime // Logging AOP에 대한 annotation
    public String processCreationForm(@Valid Owner owner, BindingResult result) {
        ...
    }
}
```

