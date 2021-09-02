import assert from 'assert';
import { MongoClient } from 'mongodb';

let _client: MongoClient;
let _clientPromise: Promise<MongoClient>;

export function initClient(uri: string): void {
  if (_client) {
    console.warn('Trying to init DB again!');
    return;
  }

  _client = new MongoClient(uri);
  _clientPromise = _client.connect();
}

export async function getClient(): Promise<MongoClient> {
  assert.ok(_clientPromise, 'Db has not been initialized. Please called init first.');
  return _clientPromise;
}
