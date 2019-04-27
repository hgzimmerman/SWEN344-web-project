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

/**
 * Gets the jwt and
 * @returns {*}
 */
export function getJwtBearer() {
    let jwt = getJwt();
    if (jwt !== null) {
        return 'bearer ' + getJwt();
    } else {
        console.error("Trying to construct bearer string for authentication while not logged in.");
        throw new Error("Attempt to get JWT bearer, but JWT does not exist");
    }
}

/**
 * Creates a promise for a fetch request that has been initialized with the jwt token.
 * @param url
 * @param extras A object containing the body and method. It is nullable and not required for GET requests.
 *        method Must be of type "put" | "post" | "get" | "delete" | "patch". It is not nullable.
 *        body Must be of type string | null.
 */
function authenticatedFetch(url, extras) {
    let headers = {
      "Authorization": getJwtBearer(),
      'content-type': 'application/json',
    };
    if (extras !== undefined && extras.method !== undefined && extras.body !== undefined) {
      console.log("making a request with a body");
      let init = {
        headers,
        body: extras.body,
        method: extras.method
      };
      return fetch(url, init)
    } else {

      console.log("making request without body");
      return fetch(url, {headers})
    }
}

/**
 * Authenticates a request, then deserializes the response and handles the error.
 * @param url
 * @returns {*}
 */
export function authenticatedFetchDe(url, extras) {
  return authenticatedFetch(url, extras)
    .then(response => {
      let json = response.json();
      if (response.ok) {
        return json;
      } else {
        return json.then(err => {throw err;});
      }
    })
}

export const AuthButton = withRouter(
  ({ history }) => (
      <MenuItem
        id='signoutBtn'
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
