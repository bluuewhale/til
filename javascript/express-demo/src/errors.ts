export class MyError extends Error {
  status: number;

  constructor(message: string, status = 500) {
    super(message);
    this.status = status;
  }
}

export function isMyError(err: Error | MyError): err is MyError {
  return Boolean((<MyError>err).status);
}
