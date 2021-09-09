import { authService } from 'fbase';
import {
  createUserWithEmailAndPassword,
  signInWithEmailAndPassword,
} from 'firebase/auth';
import React, { useState } from 'react';

function Auth() {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [newAccount, setNewAccount] = useState(false);
  const [error, setError] = useState('');

  const onChange = (event) => {
    const {
      target: { name, value },
    } = event;

    if (name === 'email') {
      setEmail(value);
    } else if (name === 'password') {
      setPassword(value);
    }
  };

  const onSubmit = async (event) => {
    event.preventDefault();

    let credential;
    try {
      if (newAccount) {
        credential = await createUserWithEmailAndPassword(
          authService,
          email,
          password
        );
      } else {
        credential = await signInWithEmailAndPassword(
          authService,
          email,
          password
        );
      }

      console.log(credential);
    } catch (err) {
      setError(err.message);
    }
  };

  return (
    <div>
      <form onSubmit={onSubmit}>
        <input
          name="email"
          type="email"
          placeholder="Email"
          required
          onChange={onChange}
          value={email}
        />
        <input
          name="password"
          type="password"
          placeholder="Password"
          required
          onChange={onChange}
          value={password}
        />
        <input
          name="submit"
          type="submit"
          value={newAccount ? 'Sign Up' : 'Sign In'}
          required
        />
      </form>
      <div>
        <button>Continue with Google</button>
        <button>Continue with Github</button>
      </div>
      <div>{error}</div>
    </div>
  );
}

export default Auth;
