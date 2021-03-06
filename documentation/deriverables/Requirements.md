# Requirements Document
Team 3: Beltran Caliz, Sadaf Chowdhury, Dylan Forrest, John King, Henry Zimmerman

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

![Main](../wireframes/MainPageWireframe.png){ width=350px }

![Main Alternative](../wireframes/MainPageWireframe2.png){ width=350px }

![Stocks](../wireframes/Stock.png){ width=350px }

![Calendar](../wireframes/Calendar.png){ width=350px }

\newpage

### At least 5 projects risks - with a 1-2 paragraph write up (for each risk) of how you plan to address these risks
##### Rust:

The backend api, being written in Rust, is not able to be worked upon by most team members.
This means that most server development will fall on one member, possibly bottlenecking the project.
- Solution: Have the backend mostly done before frontend works starts.

##### Warp:

The backend, written in a functional web framework, has unfamiliar semantics with respect to how consumers of the api could read the code and determine what a given endpoint needs.
- Solution: Have a separate, up to date document with the details necessary to understand the API. Also, explore keeping the API function calls in Rust code that can be compiled to WASM and exported to the frontend, simplifying the work required by the frontend team.

##### Stock Trading API:

We may not be able to find a good stock info API for our needs.
We need a way to search and visualize the stock data, and ideally present a list of all stocks available for purchase up front.
- Solution: Identify early a possible stock info API, and modify the backend to augment what it isn't readily capable of.

##### Developer Environment:

Since our computers span all of the popular OS's, setting up a developer environment that works across all of them may be challenging. The backend requires that Postgres, lib Postgres, lib OpenSSL, and Rust are installed, as well as specific environment variables are present.
- Solution: Finish and host the backend ASAP, so the frontend can be developed without the backend running locally.
Failing that, there will exist documentation covering what needs to be done to set up an environment.
Also, a `default.nix` file will exist, which when run with the nix package manager, will set up the development environment as needed on supported OSs (Linux + MacOS + Linux Subsystem for Windows).
Docker may be explored if necessary.

##### Testing:
With the minimal non-sql logic in the backend, there will be few unit tests to cover the behavior of the code.
Also, running the integration tests on a CI suite may prove difficult because of a dependency on a database being run as well.
- Solution: We will cover the route matching as unit tests, and be prepared to run integration tests locally if required.




### At least 15 good user stories
1. I as a user, want to be able to log in to the app using my Twitter Account to post or read tweets.
2. I as a user, want to be able to view some of my stocks on the main page so that I may keep track of my gains and losses.
3. I as a user, want to be able to quickly access all of the main components of the app using a navigational header so that I may navigate around the site.
4. I as a user, want to purchase and sell stocks so that I may keep track of my gains and losses.
5. I as a user, want to download purchase and sales history so that I can keep track of my history.
6. I as a user, want to see a history of my stocks so that I can see how much money I made or lost.
7. I as a user, want to be able to see today's calendar events on the main screen so that I can keep track of upcoming events.
8. I as an admin or curious user, want to be able to see the history of the server availability, load, and weather the "adds" were served so that I can keep track of the site's performance.
9. I as a user, want to be able see a whole month's worth of calendar events for the current month so that I can keep of upcoming events.
9. I as a user, want to be able see calendar events for future and prior months so that I can keep track of upcoming events and make future plans.
10. I as a user, want to create calendar events so that I can create reminders for myself.
11. I as a user, don't want to see other user's events or stock trades so that I can keep my info confidential.
12. I as a user, want to be able to export all of my events, so a friend can import them as their own so that we can both keep track of upcoming events.
13. I as a user, want a to view part of my twitter feed on the main page so that I can keep track of current tweets.
14. I as a user, want to cancel calendar events so that I can keep my calendar clean from unneeded events.
15. I as a user, want to query available stocks so that I can have the option of purchasing them.
