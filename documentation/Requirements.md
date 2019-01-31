### At least 10 use cases

1. Create Facebook statuses.
2. Look at Facebook statuses.
3. Create Calendar events.
4. See upcoming Calendar events.
5. Modify Calendar events.
6. Look at stocks.
7. Add "funds" required to buy stocks.
8. Purchase/Sell stocks.
9. See net gains/losses from stock trading.
10. See stock transaction history.
11. Log in via Facebook OAuth.

### Include wireframes for at least the following pages:
#### Main home page with features including:
        
##### Top 5 stocks

##### Facebook/Twitter Information

#### Stocks page

#### Calendar
    
    
    
### At least 5 projects risks - with a 1-2 paragraph write up (for each risk) of how you plan to address these risks
1. ##### Rust:

The backend api, being written in Rust, is not able to be worked upon by most team members.
This means that most server development will fall on one member, possibly bottlenecking the project.
- Solution: Have the backend mostly done before frontend works starts.

2. ##### Warp:

The backend, is written in a functional web framework, has unfamiliar semantics with respect to how consumers of the api could read the code and determine what a given endpoint needs.
- Solution: Have a separate, up to date document with the details necessary to understand the API. Also, explore keeping the API function calls in Rust code that can be compiled to WASM and exported to the frontend, simplifying the work required by the frontend team.

3. ##### Stock Trading API:

We may not be able to find a good stock info API for our needs.
We need a way to search and visualize the stock data, and ideally present a list of all stocks available for purchase up front.
- Solution: Identify early a possible stock info API, and modify the backend to augment what it isn't readily capable of.

4. ##### Developer Environment:

Since our computers span all of the popular OS's, setting up a developer environment that works across all of them may be challenging. The backend requires that Postgres, lib Postgres, lib OpenSSL, and Rust are installed, as well as specific environment variables are present.
- Solution: Finish and host the backend ASAP, so the frontend can be developed without the backend running locally.
Failing that, there will exist documentation covering what needs to be done to set up an environment.
Also, a `default.nix` file will exist, which when run with the nix package manager, will set up the development environment as needed on supported OSs (Linux + MacOS + Linux Subsystem for Windows).
Docker may be explored if necessary.

5. ##### Testing:
With the minimal non-sql logic in the backend, there will be few unit tests to cover the behavior of the code.
Also, running the integration tests on a CI suite may prove difficult because of a dependency on a database being run as well.
- Solution: We will cover the route matching as unit tests, and be prepared to run integration tests locally if required.




### At least 15 good user stories
1.
2.
3.
4.
5.
6.
7.
8.
9.
10.
11.
12.
13.
14.
15.
