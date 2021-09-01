const assert = require("assert");
const { MongoClient } = require("mongodb");

let _client;

function initialize(uri, callback) {
  if (_client) {
    console.warn("Trying to init DB again!");
    return callback(null, _client);
  }
  MongoClient.connect(uri, function (err, db) {
    if (err) {
      return callback(err);
    }
    console.log("DB initialized - connected to: " + uri.split("@")[1]);

    _client = db;
    return callback(null, _client);
  });
}

function get() {
  assert.ok(_client, "Db has not been initialized. Please called init first.");
  return _client;
}

exports.initialize = initialize;
exports.get = get;
