
# 1: Build the exe
FROM rust:1.48 as builder
WORKDIR /usr/src

# 1a: Prepare for static linking
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

# 1b: Download and compile Rust dependencies (and store as a separate Docker layer)
RUN USER=root cargo new snmp-actix
WORKDIR /usr/src/snmp-actix
COPY Cargo.toml Cargo.lock build.rs ./
RUN cargo install --target x86_64-unknown-linux-musl --path .

# 1c: Build the exe using the actual source code
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# 2: Copy the exe and extra files ("static") to an empty Docker image
FROM debian:buster-slim

ARG APP=/usr/src/app
ENV TZ=Etc/UTC \
    APP_USER=appuser


RUN apt-get update && apt-get install -y  netcat

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP} \
    && mkdir -p ${APP}/config

COPY --from=builder /usr/local/cargo/bin/snmpcollector ${APP}

COPY config/* ${APP}/config/
COPY entrypoint.sh ${APP}

RUN chown -R $APP_USER:$APP_USER ${APP}
RUN chmod +x ${APP}/entrypoint.sh
EXPOSE 8161-8185

USER $APP_USER
WORKDIR ${APP}


CMD ["./entrypoint.sh"]

