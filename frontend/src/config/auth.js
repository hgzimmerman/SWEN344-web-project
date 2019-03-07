import React from 'react';
import MenuItem from '@material-ui/core/MenuItem';
import { Route, Redirect, withRouter } from 'react-router-dom';

export const PrivateRoute = function PrivateRoute({ component: Component, ...rest }) {
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

export const AuthButton = withRouter(
  ({ history }) => (
      <MenuItem
        onClick={() => {fakeAuth.signout(() => history.push("/"))}}
      >
        Logout
      </MenuItem>
    )
);
