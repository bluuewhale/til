export interface MyError extends Error {
  status?: number;
}

export class MyError extends Error {
  constructor(message: string, status: number = 500) {
    super(message);
    this.status = status;
  }
}

export function isMyError(err: Error | MyError): err is MyError {
  return Boolean((<MyError>err).status);
}
