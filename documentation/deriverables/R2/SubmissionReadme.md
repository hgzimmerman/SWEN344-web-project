## Submission Readme
#### Site URL
[Site URL](https://vm344c.se.rit.edu/login)

#### Account
An account for our system will automatically be created once you sign in using your Twitter account.


#### Readme
The project at the moment supports at least the following:
* Sign in via Twitter OAuth.
* Stock visualization.
* Calendar visualization.
* Main page.
* Current Weather (with customizable zip code).
* Automatic User account creation.
* Authentication via JWT.
* Stock purchase tracking.
* Stock profit/loss calculation.
* ~~Calendar event creation, retrial, modification, deletion.~~
* Adaptive advertisement serving + logging + health history retrial and visualization.


Not Implemented:
* Multiple "Environments" (`/R2/...` vs `/R1/...`). We decided against doing this because:
  1. This is never something you would do in real life (you would have a separate host or port, not hosting a different frontend that would connect to a now removed api).
  2. The backend person who would have worked on this was pulled into doing frontend work in order to get anything done for this release.
  3. This was considered low priority because without a formal rubric, we didn't see how this would affect our grades. We deemed this as low priority.
* Frontend calendar event integration with backend.
  * Backend api exists and is tested.
  * Frontend implementation currently crashes.