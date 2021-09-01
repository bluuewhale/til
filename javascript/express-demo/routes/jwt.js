const jwt = require('jsonwebtoken');
const config = require('../config');

function getTokenFromHeader(req) {
  if (
    (req.headers.authorization &&
      req.headers.authorization.split(' ')[0] === 'Token') ||
    (req.headers.authorization &&
      req.headers.authorization.split(' ')[0] === 'Bearer')
  ) {
    return req.headers.authorization.split(' ')[1];
  }

  return null;
}

function signToken(payload, secret, cb) {}

// Custom Authentication Middleware
function verifyToken(req, res, next) {
  token = getTokenFromHeader(req);
  secret = req.app.get('jwt-secret');
  console.log(token);
  console.log(secret);

  jwt.verify(token, secret, function (err, decoded) {
    if (err) {
      throw err;
    }

    req.decoded = decoded;
    next();
  });
}

module.exports = {
  signToken: signToken,
  verifyToken: verifyToken,
};
