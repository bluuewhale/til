import type { Request, Response, NextFunction } from 'express';
import jwt from 'jsonwebtoken';
import { MyError } from '../errors';
import { RequestWithUsername } from '../types/common';

function getTokenFromHeader(req: Request): string | null {
  if (
    (req.headers.authorization && req.headers.authorization.split(' ')[0] === 'Token') ||
    (req.headers.authorization && req.headers.authorization.split(' ')[0] === 'Bearer')
  ) {
    return req.headers.authorization.split(' ')[1];
  }

  return null;
}

// Custom Authentication Middleware
export function verifyToken(req: RequestWithUsername, _res: Response, next: NextFunction): void {
  const token = getTokenFromHeader(req);
  const secret = req.app.get('jwt-secret');

  if (!token) {
    throw new MyError('Missing Authorization headers', 400);
  }

  try {
    const { username } = jwt.verify(token, secret) as { username: string };

    req.username = username;
    next();
  } catch (err) {
    throw new MyError(err.message || 'Authorization failed', 401);
  }
}
