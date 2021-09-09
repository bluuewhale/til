const images = Array.from({ length: 10 }, (x, i) => `img/${i}.jpg`);

function getRandomImg(images) {
  const randomIdx = Math.floor(Math.random() * images.length);
  return images[randomIdx];
}

function createBgImg() {
  const img = document.createElement('img');
  img.src = getRandomImg(images);
  img.classList.add('bg');

  return img;
}

const randomImg = getRandomImg(images);
document.body.style = `background-image:url(${randomImg}); background-size: cover;`;
document.body.classList.add('background');
