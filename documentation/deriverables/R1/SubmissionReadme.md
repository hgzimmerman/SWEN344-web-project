## Submission Readme
##### Site URL
[http://vm344c.se.rit.edu/](http://vm344c.se.rit.edu/)

##### Login Information
An account for our system will automatically be created once you sign in using your Facebook account.
Depending on time allocated, full login may not be implemented.
Instead, a user account will be generated regardless of provided credentials.
A compile time variable is used to switch between using a future that contacts Facebook, or creating a dummy account.

Because of a lack of time, we have not adequately established that OAuth works as intended, so we are defaulting to the dummy account.

##### Readme
As per R1 specifications, the project at the moment supports at least the following:
* Sign in via OAuth provider.
* Stock visualization.
* Calendar visualization.
* Main page setup.
* Current Weather.

Backend support in excess of R1 specifications exists for:
* User account creation.
* Authentication.
* Stock purchase tracking.
* Stock profit/loss calculation.
* Calendar event creation, retrial, modification, deletion.
* Adaptive advertisement serving + logging + health history retrial.
