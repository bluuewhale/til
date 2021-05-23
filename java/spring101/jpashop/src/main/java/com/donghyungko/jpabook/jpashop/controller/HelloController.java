import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;

@Controller
public class HelloController {

    @GetMapping("hello")
    public String hello(Model model) {
        // Model은 데이터를 실어서 View로 넘겨주는 데이터 저장 객체이다.
        model.addAttribute("data", "HELLO!!");
        return "hello";
    }
}
