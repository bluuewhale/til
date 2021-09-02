import type { Request } from 'express';

export interface RequestWithUsername extends Request {
  username?: string;
}
