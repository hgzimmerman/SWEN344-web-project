# API Documentation

### Document Purpose
This document should cover what each api endpoint does and how to call it.


### Brief Overview
The API is any endpoint with a route starting with `/api/` on server.
The API is a mostly standard REST affair, with some minor inconsistencies in how some things are named, and that PUT requests often will not require the full contents of the object being changed, only the relevant parts being changed.

Except for things involving initial login requests, the API uses UUIDs instead of IDs.
If you are unfamiliar with UUIDs more info can be found here: https://en.wikipedia.org/wiki/Universally_unique_identifier . 
Quickly put, UUIDs are randomly generated identifiers that are long enough to statistically ensure that there will never be a collision, and are therefore all unique. 

The API uses JWTs for authenticating between itself and the application.
If you are unfamiliar with JWTs, more info can be found here: https://jwt.io/introduction/ .
JWTs are a a way of managing user sessions using a cryptographically verifiable token.
This token contains the a user's UUID and should be the only source of truth regarding which user the request should apply to.

This API eschews OAuth for all purposes except the initial login.
That may change, should the API need to make requests to the OAuth provider's service, but those calls should be able to be made from the frontend application instead 


### JWT Details
For the purposes of the client, the JWT is a magic string.
It never _needs_ to be inspected or modified, but it can be with some base64 decoding.
What it does need is to be attached to any request that requires authentication (which is most requests).
This is done by attaching it to the authorization header.
It should look something like this (where 'garbage' is a stand-in for the random characters that comprise a JWT):
```
Authorization: bearer garbage.garbage.garbage 
```

When you get the response from the login api endpoint, take the plain string and store it in `localstorage`.
This allows it to be accessible from a global context within the app, as well as on other tabs in the same web browser.

### File Serving Details
To work with a routed SPA client in the browser, when requesting a non-`/api/*` route on the server, the server will return the requested file or the `index.html` if the file can't be found.
This intentionally breaks with the expected behavior of returning a `404` response.

This is done to allow the user to visit the site at an arbitrary path (eg. `http://my-app.com/login/`), have the server fail to find the file named `login` and instead return the `index.html`
By returning the `index.html` file, the browser will then request the css and js required to run the app, the router will detect that the url still points to `http://my-app.com/login/` and route to the login page/component.

### API

###### Note
The `/:` syntax indicates that the segment in the route is an arbitrary string that can be coerced to that type.

The 'Requires Auth' column indicates if the request needs the JWT attached to the request.

####Implemented


| Route                               | Method | Body Return Type   | Body Accept Type    | Requires Auth |Description                            |
| -------------------------------     | ------ | ----------------   | ------------------  | ------------- |-------------------------------------- |
| `/:filepath`                        | GET    | file resource      |                     | no            | Gets the requested file, and failing that - returns index.html instead of a 404 |
| `/api/auth/login/`                  | POST   | String             | Login               | no            | Logs in to the application, returning JWT string |
| `/api/auth/user/`                   | GET    | User               |                     | yes           | Gets the user                         |
| `/api/calendar/event/events`        | GET    | \[Event\]          |                     | yes           | Gets all events for user              |
| `/api/calendar/event/events/today`  | GET    | \[Event\]          |                     | yes           | Gets events today for user            |
| `/api/calendar/event/events/month`  | GET    | \[Event\]          |                     | yes           | Gets events this Month for user       |
| `/api/calendar/event/:uuid`         | DELETE | Event              |                     | yes           | Deletes event                         |
| `/api/calendar/event/`              | POST   | Event              | NewEventMessage     | yes           | Creates event                         |
| `/api/calendar/event/`              | PUT    | Event              | EventChangeset      | yes           | Modifies event                        |
| `/api/market/stock/`                | GET    | \[StockResponse\]    |                   | yes           | All the stocks the user owns          |
| `/api/market/stock/transact`        | POST   | StockTransaction  | StockTransactionRequest | yes        | Buys or sells a quantity of a given stock|
| `/api/market/stock/transactions/:stockName`| GET | \[StockTransaction\] |               | yes           | Gets the transaction history for a given stock |
| `/api/market/stock/performance`      | GET    | \[(f64, Stock)\]     |                  | yes           | Gets the performance for each stock the user has  |

#### UNIMPLEMENTED                      
###### Instructions
When one of these is implemented, move it to the upper table.                      
                      
| Route                               | Method | Body Return Type   |  Body Accept Type   | Requires Auth |Description                            |
| -------------------------------     | ------ | ----------------   | ------------------- | ------------- |-------------------------------------- |
| `/api/advertisement`                | GET    | `.png`             |                     | false         | Gets the advertisement if available   |
| `/api/health`                       | GET    | \[HealthRecord\]   |                     | false         | Gets all of the history of requests for the advertisement      |
| `/api/health/week`                  | GET    | \[HealthRecord\]   |                     | false         | Gets the last weeks worth of the history of requests for the advertisement      |


