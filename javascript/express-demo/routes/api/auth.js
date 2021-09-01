const router = require('express').Router();
const jwt = require('jsonwebtoken');
const Joi = require('joi');

router.post('/login', async function (req, res) {
  schema = Joi.object().keys({
    username: Joi.string().required(),
    password: Joi.string().required(),
  });

  var { error, value } = schema.validate(req.body);
  if (error) {
    throw error;
  }

  if (!(value.username == 'admin') || !(value.password == 'admin')) {
    throw new Error('Invalid username or password');
  }

  var payload = { username: value.username };
  var secret = req.app.get('jwt-secret');
  var opt = {
    expiresIn: '30m',
    issuer: 'koko8624.com',
    subject: 'userInfo',
  };

  jwt.sign(payload, secret, opt, function (err, token) {
    if (err) {
      throw err;
    }

    res.json({ token: token });
  });
});

module.exports = router;
