import React from 'react';
import MenuItem from '@material-ui/core/MenuItem';
import { Route, Redirect, withRouter } from 'react-router-dom';

export const PrivateRoute = function PrivateRoute({ component: Component, ...rest }) {
  return (
    <Route
      {...rest}
      render={props =>
        isAuthenticated() ? (
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
};
const JWT_KEY = 'jwt';

export function isAuthenticated() {
    return getJwt() != null;
}


export function signout() {
    window.localStorage.removeItem(JWT_KEY);
}
/**
 * Gets the JWT string.
 *
 * Returns null if JWT is not present.
 */
export function getJwt() {
    return window.localStorage.getItem(JWT_KEY);
}

export function getJwtBearer() {
    let jwt = getJwt();
    if (jwt !== null) {
        return 'bearer ' + getJwt();
    } else {
        console.error("Trying to construct bearer string for authentication while not logged in.");
        return null
    }
}

export const AuthButton = withRouter(
  ({ history }) => (
      <MenuItem
        onClick={
          () => {
              signout(); // Remove the JWT from storage
              // Kick off a state change + re-render by changing the history's location,
              // forcing the rendering logic to acknowledge the removal of the JWT from storage,
              // indicating that the user is now signed out.
              // This will cause a redirect to /login.
              history.push("/") // TODO, it may be better to make this explicitly point to /login.
            }
        }
      >
        Logout
      </MenuItem>
    )
);
