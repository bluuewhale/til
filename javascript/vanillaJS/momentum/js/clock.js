const clock = document.querySelector('h2#clock');

function setClock() {
  const now = new Date();
  const hours = now.getHours().toString().padStart(2, '0');
  const minutes = now.getMinutes().toString().padStart(2, '0');
  const seconds = now.getSeconds().toString().padStart(2, '0');
  const nowString = `${hours}:${minutes}:${seconds}`;

  clock.innerText = nowString;
}

setClock();
setInterval(setClock, 1000);
