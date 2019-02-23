# SWEN344-web-project
Group project for RIT SWEN344


### Deploy

To create the environment in which the project can build and run, you need both docker and docker-compose installed.

Run `docker-compose -f docker-compose.yml up` to bring up the containers required by the project.

Once running, you can inspect a container by running `docker container list`, finding the container id, and substituting it in the following command: `docker exec -ti <ID> /bin/bash`

