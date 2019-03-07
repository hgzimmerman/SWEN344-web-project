## Bug Tracking

#### Methodology
Bug tracking was handled via the same interface as our features.
This was done through GitHub's Kanban-style projects board feature, accessible [here](https://github.com/hgzimmerman/SWEN344-web-project/projects/).

As bugs were discovered, issues corresponding to the bug were entered into the project board in the `To Do` column.
As they were claimed for work. they would be moved to the `In Progress` column.
When fixed, they would be moved to the `Done` column.

### Bug Counts and Analysis

###### Backend
The backend was mostly finished and tested by the time dependent work on the frontend started.
As consequence, few bug reports were lodged against it.
When they were, they were quickly addressed because there wasn't concurrent development.

###### Frontend


#### How many bugs did you collect?
1. Creating a post does not update the homepage feed
2. Creating a post does not update the user's Facebook feed on facebook.com
3. Entering blank for the create a Facebook post still creates a Facebook post
4. Entering blank and clicking search for the stocks search bar does not yield a warning or error
5. Resizing the input textbox for the Facebook post gets layered under the other web components on the same page.
6. Logging into the site using FB authentication while already having a Facebook account logged in yields the following error: "App Not Setup: This app is still in development mode, and you don't have access to it. Switch to a registered test user or ask an app admin for permissions." User most go to incognito/private mode to log in.

#### How many open bugs do you have? And why are they open?


#### How has the bug collection/mitigation process helped or hurt your project.
There is an inherent time overhead to tracking bugs in an informal manner instead of just telling each other to fix them when we meet.
Formally keeping track of them provides a means to ensure that each bug is addressed or at least known about.
