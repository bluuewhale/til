const loginInput = document.querySelector("#login-form input");
const loginButton = document.querySelector("#login-form button");

console.log(loginButton);

function onLoginBtnClick() {
    const username = loginInput.value;
    alert("Hi" + username);
}
loginButton.addEventListener("click", () => {
    onLoginBtnClick();
})