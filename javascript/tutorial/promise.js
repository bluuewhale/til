function getData() {
  return new Promise(function (resolve, reject) {
    setTimeout(function () {
      var data = 0;
      resolve(data);
    }, 2000);
  });
}

function getDataFailure() {
  return new Promise(function (resolve, reject) {
    setTimeout(function () {
      reject(new Error("Data not found"));
    });
  });
}

function printData(data) {
  console.log(data);
}

function printError(err) {
  console.log(err);
}

//getData().then(printData).catch(printError);
//getDataFailure().then(printData).catch(printError);

///////////////////////////////////////////////////////////

const USER_INFO_DB = {
  foo: "bar",
};

function getUserInfo() {
  return new Promise(function (resolve, reject) {
    setTimeout(function () {
      userInfo = {
        name: "foo",
        password: "bar",
      };
      resolve(userInfo);
    }, 2000);
  });
}

function validateUserInfo(userInfo) {
  return new Promise(function (resolve, reject) {
    if (!("name" in userInfo) || !("password" in userInfo)) {
      reject(new Error("missing key name or password"));
    }

    resolve(userInfo);
  });
}

function checkUserInfoMatch(userInfo) {
  return new Promise(function (resolve, reject) {
    name = userInfo["name"];
    pw = userInfo["password"];
    if (pw === USER_INFO_DB[name]) {
      resolve(name);
    }
    reject(new Error("password does not match"));
  });
}

function successLogin(username) {
  console.log(`user ${username} is successfully logged in`);
}

getUserInfo()
  .then(validateUserInfo)
  .then(checkUserInfoMatch)
  .then(successLogin)
  .catch(printError);
