const todoForm = document.querySelector('#todo-form');
const todoInput = todoForm.querySelector('input');
const todos = document.querySelector('#todo-list');

const TODO_KEY = 'todo';
let todoList;

class Todo {
  constructor(txt = new Array()) {
    this.id = Date.now();
    this.txt = txt;
  }
}

class TodoList {
  constructor(inner) {
    this.inner = inner;
  }

  push(todo) {
    console.dir(this);
    console.log(this.inner);
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
  todoList = new TodoList();

  let val = localStorage.getItem(TODO_KEY);
  if (val != null) {
    todoList.inner = JSON.parse(val);
    todoList.inner.forEach(displayTodo); // update screen
  }
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
    todoList.removeById(todo.id);
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
  todoList.push(todo);
}

// load on screen
loadTodoList();
todoForm.addEventListener('submit', submitTodoForm);
