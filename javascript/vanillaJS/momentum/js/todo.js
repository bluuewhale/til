const todoForm = document.querySelector('#todo-form');
const todoInput = todoForm.querySelector('input');
const todos = document.querySelector('#todo-list');

const TODO_KEY = 'todo';
let TODO_LIST;

class Todo {
  constructor(txt = new Array()) {
    this.id = Date.now();
    this.txt = txt;
  }
}

class TodoList {
  constructor(inner) {
    if (inner === undefined) {
      this.inner = new Array();
    } else {
      this.inner = inner;
    }
  }

  push(todo) {
    this.inner.push(todo);
    this.save();
  }

  save() {
    localStorage.setItem(TODO_KEY, JSON.stringify(this.inner));
  }

  removeById(id) {
    this.inner.some((todo, idx) => {
      if (todo.id == id) {
        this.inner.splice(idx, 1);
      }

      this.save();
    });
  }
}

function loadTodoList() {
  TODO_LIST = new TodoList();

  let val = JSON.parse(localStorage.getItem(TODO_KEY));
  if (val != null) {
    TODO_LIST.inner = val;
    TODO_LIST.inner.forEach(displayTodo); // update screen
  }
  console.dir(TODO_LIST);
}

function displayTodo(todo) {
  const li = document.createElement('li');
  const span = document.createElement('span');
  span.innerText = todo.txt;

  const button = document.createElement('button');
  button.innerText = '❌';
  button.addEventListener('click', (event) => {
    event.preventDefault();

    // update binded context
    li.remove();
    TODO_LIST.removeById(todo.id);
  });

  li.appendChild(span);
  li.appendChild(button);
  todos.appendChild(li);
}

function submitTodoForm(event) {
  event.preventDefault();

  const todo = new Todo(todoInput.value);

  // update
  displayTodo(todo);
  TODO_LIST.push(todo);
}

// load on screen
loadTodoList();
todoForm.addEventListener('submit', submitTodoForm);
