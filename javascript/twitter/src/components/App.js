import { useEffect, useState } from 'react';
import './App.css';
import Router from 'components/Router';
import { authService } from 'fbase';

function App() {
  const [init, setInit] = useState(false);
  const [isLoggedIn, setIsLoggiedIn] = useState(false);

  useEffect(() => {
    authService.onAuthStateChanged((user) => {
      if (user) {
        setIsLoggiedIn(true);
      } else {
        setIsLoggiedIn(false);
      }
      setInit(true);
    });
  });

  return init ? (
    <>
      <Router isLoggedIn={isLoggedIn} />
    </>
  ) : (
    <span>Initializing...</span>
  );
}

export default App;
