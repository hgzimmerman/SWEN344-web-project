FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features postgres

RUN curl -sL https://deb.nodesource.com/setup_10.x | bash
RUN apt update -y
RUN apt install nodejs -y
# RUN apt install certbot -y

WORKDIR /usr/src/app/frontend

#RUN ls
#RUN ls
#WORKDIR /usr/src/app
#RUN ls
#
#
#RUN npm install
#RUN npm run-script build


WORKDIR /usr/src/app/backend

EXPOSE 80
EXPOSE 443

VOLUME ["/usr/local/cargo"]

