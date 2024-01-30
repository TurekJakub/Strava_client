FROM rust:1.73

WORKDIR /usr/src/api

COPY ./api ./api
COPY ./strava-client ./strava-client
COPY ./.env /

WORKDIR /usr/src/api/api

RUN cargo install --path .

EXPOSE 8080
ENTRYPOINT ["cargo"]