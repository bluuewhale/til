import express, { Response, Request, NextFunction } from 'express';
import compression from 'compression';
import morgan from 'morgan';
import chalk from 'chalk';
import { config } from 'dotenv';

import { initClient } from './db';
import router from './routes/api';
import { isMyError, MyError } from './errors';

// Envs
config();

const secret = process.env.JWT_SECRET;
if (!secret) throw new Error('Missing Env JWT_SECRET!!!');

const mongoUri = process.env.MONGO_URI;
if (!mongoUri) throw new Error('Missing Env MONGO_URI!!!');

const port = process.env.PORT || 3000;

// Logger
const morganM = morgan((tokens, req, res) => {
  return [
    chalk.hex('#34ace0').bold(tokens.method(req, res)),
    chalk.hex('#ffb142').bold(tokens.status(req, res)),
    chalk.hex('#ff5252').bold(tokens.url(req, res)),
    chalk.hex('#2ed573').bold(`${tokens['response-time'](req, res)} ms`),
    chalk.hex('#f78fb3').bold(`@ tokens.date(req, res)`),
    chalk.yellow(tokens['remote-addr'](req, res)),
    chalk.hex('#fffa65').bold(`from ${tokens.referrer(req, res)}`),
    chalk.hex('#1e90ff')(tokens['user-agent'](req, res)),
  ].join(' ');
});

// middleware
const app = express();
app.use(compression());
app.use(morganM);
app.use(express.json()); // parse json body
app.use(express.urlencoded({ extended: true })); // parse form

// routes
app.use('/api', router);

// 404 handler
app.all('*', (_req, _res, next) => {
  next(new MyError('method not exists', 404));
});

// error handler, no stacktraces leaked to user
// eslint-disable-next-line @typescript-eslint/no-unused-vars
app.use((err: Error | MyError, _req: Request, res: Response, _next: NextFunction) => {
  if (isMyError(err)) {
    return res.status(err.status || 500).send(err.message || 'internal error');
  }

  res.status(500).send(err.message || 'internal error');
});

// JWT
app.set('jwt-secret', secret);

// Mongo
initClient(mongoUri);

app.listen(port, () => {
  console.log(`API Up and running on port ${port}`);
});
