package com.donghyungko.spring101.helloworld;

/**
 * Hello world!
 *
 */
@WebServlet("/helloworld")
public class HelloWorld extends HttpServlet {
    private static final long serialVersionUID = 1L;

    public HelloWorld() {
        super();
    }

    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws ServletException, IOException {
        response.setContentType("text/html;charset=UTF-8");
        PrintWriter out = response.getWriter();
        out.print("<h1>Hello World!</h1>");
    }
}
