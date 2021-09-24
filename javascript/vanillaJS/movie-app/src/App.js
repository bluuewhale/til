import React from 'react';
import { BrowserRouter, Route } from 'react-router-dom';
import Navigation from './routes/Navigation';
import Home from './routes/Home';
import About from './routes/About';
import MovieDetail from './routes/MovieDetail';

function App() {
  return (
    <BrowserRouter>
      <Navigation />
      <Route path="/" exact={true} component={Home} />
      <Route path="/about" component={About} />
      <Route path="/movie/:id" component={MovieDetail} />
    </BrowserRouter>
  );
}

export default App;
