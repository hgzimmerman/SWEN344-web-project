# SWEN344-web-project
Group project for RIT SWEN344.

https://www.se.rit.edu/~swen-344/projects/projectdescription.html#R1


### Deployment

To create the environment in which the project can build and run, you need both `docker` and `docker-compose` installed.

SSH credentials are starred in the Slack.

You need to bring down the docker containers to force a rebuild if they are already running.
Run `docker-compose -f docker-compose.yml down` to bring down the containers and network.
To bring them up, and build any changes, run `docker-compose -f docker-compose.yml up -d`.
Auditing the build process can be done using `docker-compose -f docker-compose.yml logs`, which will print out the logged stdout for all running containers up until the point where the command was invoked.

It takes about 10 minutes for the whole thing to build between the NPM build process, rustc being dog-slow, and the server being split between like 10 other VMs (we are probably only given one core to work with).

Once running, you can inspect a container by running `docker container list`, finding the container id, and substituting it in the following command: `docker exec -ti <ID> /bin/bash`

To run on localhost:
`docker-compose -f docker-compose-dev.yml up -d``


### Bootstraping

To get SSL to work in the first place, you need to be able to expose some files via `nginx` to verify the authenticity of the server, but because the `nginx` container won't start without the SSL creds, you get stuck in a chicken and egg problem.
There is a [script](https://github.com/hgzimmerman/SWEN344-web-project/blob/master/init-letsencrypt.sh) that you need to run initially to circumvent this problem, which will do all the initial verification. 
It didn't work for me on the first few attempts, and then, under some circumstances that I don't remember, it did.
We will just hope that we don't need to re-bootstrap this project.
