const greeting = document.querySelector('#greeting');
const loginForm = document.querySelector('#login-form');
const loginButton = document.querySelector('#login-form button');
const loginInput = document.querySelector('#login-form input');

const HIDDEN_CLASS_NAME = 'hidden';
const USERNAME_KEY = 'username';

function showLoginForm() {
  if (loginForm.classList.contains(HIDDEN_CLASS_NAME)) {
    loginForm.classList.remove(HIDDEN_CLASS_NAME);
  }

  loginForm.addEventListener('submit', (event) => {
    event.preventDefault();
    loginForm.classList.add(HIDDEN_CLASS_NAME);

    const username = loginInput.value;
    localStorage.setItem(USERNAME_KEY, username);
    showGreeting(username);
  });
}

function showGreeting(username) {
  greeting.innerText = `Hello ${username}`;
  greeting.classList.remove(HIDDEN_CLASS_NAME);
}

// MAIN
const username = localStorage.getItem(USERNAME_KEY);
if (username == null) {
  showLoginForm();
} else {
  showGreeting(username);
}
