# SWEN344-web-project
Group project for RIT SWEN344


### Setup/deploy
* Clone repo to server or update repo to most recent master.
* run `docker build - < Dockerfile` to build the image. This should output a build hash.
* run `docker run -b -v ~/SWEN344-web-project -ti --network-host <BUILD HASH>`