const router = require('express').Router();
const jwt = require('jsonwebtoken');
const Joi = require('joi');
const utils = require('../../lib/utils');

router.post(
  '/login',
  utils.asyncWrapper(async (req, res) => {
    const schema = Joi.object().keys({
      username: Joi.string().required(),
      password: Joi.string().required(),
    });

    const { username, password } = await schema.validateAsync(req.body);

    if (username !== 'admin' || password !== 'admin') {
      throw new Error('Invalid username or password');
    }

    const payload = { username };

    const secret = req.app.get('jwt-secret');
    const opt = {
      expiresIn: '30m',
      issuer: 'koko8624.com',
      subject: 'userInfo',
    };

    jwt.sign(payload, secret, opt, (err, token) => {
      if (err) {
        throw err;
      }

      res.json({ token });
    });
  })
);

module.exports = router;
