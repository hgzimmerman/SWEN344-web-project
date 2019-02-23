FROM ubuntu:18.04

RUN apt-get update
RUN apt install bash -y
RUN apt install build-essential curl -y
RUN curl -sL https://deb.nodesource.com/setup_10.x | bash -
RUN apt install nodejs -y
RUN apt install pkg-config -y
RUN apt install libssl-dev -y
RUN apt install clang -y
RUN apt install zlib1g-dev -y
RUN apt install libpq-dev -y
RUN apt install libssl-dev -y
RUN rm -rf /var/lib/apt/lists/*

ENV SHELL /bin/bash
USER root
ENV HOME /home/root
ENV USER root

# Install Rust
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=$HOME/.cargo/bin:$PATH

ENV DATABASE_URL='postgres://user:example@db:5432/web_engineering'
ENV TEST_DATABASE_URL='postgres://user:example@db:5432/web_engineering_test'
ENV TEST_DATABASE_NAME='web_engineering_test'
ENV DROP_DATABASE_URL='postgres://user:example@db:5432/postgres'

# Install diesel for migrations
RUN cargo install diesel_cli --no-default-features --features 'postgres' -f

# Copy project into the server
COPY ./ $HOME/

## Compile backend
## NOTE, diesel migration run should ocurr before this, but because of the docker-compose command ordering not upholding that assumption, the build may fail.
## This may nececitate making shure the schema.rs file is up to date before building the image.
#
## Considering that this can wait to run as the CMD, it may make sense to just delete this command, and wait for migration to run.
#WORKDIR $HOME/backend/server
#RUN cargo build --release

## TODO it may make sense to move this into the CMD.
# Compile frontend
WORKDIR $HOME/frontend
RUN npm install
RUN npm build

# Run backend
WORKDIR $HOME/backend/
CMD cargo run --release

