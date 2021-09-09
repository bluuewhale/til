import React from 'react';
import { BrowserRouter, Route, Switch } from 'react-router-dom';
import Auth from 'routes/Auth';
import Home from 'routes/Home';
import PropTypes from 'prop-types';

function Router({ isLoggedIn }) {
  return (
    <BrowserRouter>
      <Switch>
        {isLoggedIn ? (
          <>
            <Route exact path="/">
              <Home />
            </Route>
          </>
        ) : (
          <>
            <Route exact path="/">
              <Auth />
            </Route>
          </>
        )}
      </Switch>
    </BrowserRouter>
  );
}

Router.propTypes = {
  isLoggedIn: PropTypes.bool,
};

export default Router;
