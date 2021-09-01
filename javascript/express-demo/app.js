const express = require('express');
const compression = require('compression');
const morgan = require('morgan');
const chalk = require('chalk');
const db = require('./db');
const config = require('./config');

const port = process.env.PORT || 3000;

// Logger
const morganM = morgan(function (tokens, req, res) {
  return [
    chalk.hex('#34ace0').bold(tokens.method(req, res)),
    chalk.hex('#ffb142').bold(tokens.status(req, res)),
    chalk.hex('#ff5252').bold(tokens.url(req, res)),
    chalk.hex('#2ed573').bold(tokens['response-time'](req, res) + ' ms'),
    chalk.hex('#f78fb3').bold('@ ' + tokens.date(req, res)),
    chalk.yellow(tokens['remote-addr'](req, res)),
    chalk.hex('#fffa65').bold('from ' + tokens.referrer(req, res)),
    chalk.hex('#1e90ff')(tokens['user-agent'](req, res)),
  ].join(' ');
});

// middleware
const app = express();
app.use(compression());
app.use(morganM);
app.use(express.json()); // parse json body
app.use(express.urlencoded({ extended: true })); // parse form
app.use(require('method-override')('_method'));
app.use(require('./routes'));

// error handler, no stacktraces leaked to user
app.use(function (err, req, res, next) {
  console.log(err.stack);
  res.status(err.status || 500).send(err.message || 'internal error');
});

// JWT
app.set('jwt-secret', config['jwt-secret']);

// Mongo
db.initialize(config.mongodbUri, function (err) {
  if (err) {
    throw err;
  }

  app.listen(port, function (err) {
    if (err) {
      throw err;
    }
    console.log('API Up and running on port ' + port);
  });
});
