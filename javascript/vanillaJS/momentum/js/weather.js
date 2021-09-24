async function onGeoOk(pos) {
  const lat = pos.coords.latitude;
  const lon = pos.coords.longitude;
  const apiKey = API_KEY;
  const url = `https://api.openweathermap.org/data/2.5/weather?lat=${lat}&lon=${lon}&appid=${apiKey}&units=metric`;

  fetch(url)
    .then((response) => response.json())
    .then((data) => {
      const city = data.name;
      const weather = data.weather[0].main;
      const temp = Math.round(parseFloat(data.main.temp));

      const cityCon = document.querySelector('#weather span:first-child');
      const weatherCon = document.querySelector('#weather span:last-child');

      cityCon.innerText = city;
      weatherCon.innerText = `${weather} / ${temp}°C`;
    });
}
//navigator.geolocation.getCurrentPosition(onGeoOk);
