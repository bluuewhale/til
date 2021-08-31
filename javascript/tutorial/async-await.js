function fetchUserData() {
  return new Promise(function (resolve, reject) {
    setTimeout(function () {
      resolve({
        username: "foo",
        password: "bar",
      });
    }, 2000);
  });
}

async function logUserData() {
  userData = await fetchUserData();
  console.log(userData);
}

function fetchUserDataError() {
  return new Promise(function (resolve, reject) {
    setTimeout(function () {
      resolve(new Error("error"));
    }, 2000);
  });
}
async function logUserDataError() {
  try {
    userData = await fetchUserDataError();
  } catch (e) {
    console.log(e);
  }
  console.log(userData);
}

logUserData();
logUserDataError();
