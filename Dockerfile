FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features postgres

RUN curl -sL https://deb.nodesource.com/setup_10.x | bash
RUN apt update -y
RUN apt install nodejs -y

WORKDIR /usr/src/app/backend

EXPOSE 8080

VOLUME ["/usr/local/cargo"]

