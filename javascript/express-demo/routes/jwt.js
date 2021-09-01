const jwt = require('jsonwebtoken');
// const config = require('../config');

function getTokenFromHeader(req) {
  if (
    (req.headers.authorization && req.headers.authorization.split(' ')[0] === 'Token') ||
    (req.headers.authorization && req.headers.authorization.split(' ')[0] === 'Bearer')
  ) {
    return req.headers.authorization.split(' ')[1];
  }

  return '';
}

// Custom Authentication Middleware
function verifyToken(req, res, next) {
  const token = getTokenFromHeader(req);
  const secret = req.app.get('jwt-secret');

  if (token === '') {
    const err = new Error('Missing Authorization headers.');
    err.status = 400;

    throw err;
  }

  try {
    const { username } = jwt.verify(token, secret);

    req.username = username;
    next();
  } catch (err) {
    // TODO: error handling (eg. TokenExpired)
    throw err;
  }
}

module.exports = {
  verifyToken,
};
