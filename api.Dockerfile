FROM rust:1.75 AS build

WORKDIR /usr/src/api

COPY ./api/src ./api/src 
COPY ./api/Cargo.toml ./api/
COPY ./strava-client/src ./strava-client/src
COPY ./strava-client/Cargo.toml ./strava-client/

WORKDIR /usr/src/api/api

RUN cargo install --path .
RUN cargo build --release

FROM ubuntu:latest as production

COPY --from=build /usr/src/api/api/target/release/api /bin/

EXPOSE 80

ENTRYPOINT [ "/bin/api" ]

