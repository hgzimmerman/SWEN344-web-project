import React from 'react';
import { Route, Redirect } from 'react-router-dom';

export const PrivateRoute = function PrivateRoute({ component: Component, ...rest }) {
  if (!fakeAuth.isAuthenticated){
    alert('You must log in to view this page!')
  }
  return (
    <Route
      {...rest}
      render={props =>
        fakeAuth.isAuthenticated ? (
          <Component {...props} />
        ) : (
          <Redirect
            to={{
              pathname: "/login",
              state: { from: props.location }
            }}
          />
        )
      }
    />
  );
}

export const fakeAuth = {
  isAuthenticated: false,
  authenticate(cb) {
    this.isAuthenticated = true;
    setTimeout(cb, 100); // fake async
  },
  signout(cb) {
    this.isAuthenticated = false;
    setTimeout(cb, 100);
  }
};
