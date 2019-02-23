# SWEN344-web-project
Group project for RIT SWEN344


### Deploy

To create the environment in which the project can build and run, you need both docker and docker-compose installed.

First run `docker build --tag yeet . -f Dockerfile     `.
Then run `docker build --tag `docker build --tag postgres-local . -f DockerfilePostgres`.
Then ...

Currently, the postgres db is ephemeral, if you redeploy, the db will be wiped out.
This will ideally change
