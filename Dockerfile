FROM debian:stable-slim AS builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY ./ /app
RUN make prod

FROM debian:stable-slim

ENV DEBIAN_FRONTEND=noninteractive

ARG TOKEN

ENV DTOKEN=$TOKEN

RUN apt-get update && apt-get install libcurl4 libssl3 uuid-runtime -y

RUN uuidgen > /etc/machine-id

RUN adduser --disabled-password --gecos "" asuka
USER asuka

WORKDIR /app

RUN mkdir -p /app/assets && chown -R asuka:asuka /app

COPY --chown=asuka:asuka --from=builder /app/Settings.toml /app
COPY --chown=asuka:asuka --from=builder /app/assets/jueves.gif /app/assets
COPY --chown=asuka:asuka --from=builder /app/target/release/asuka /app

RUN echo $DTOKEN | ./asuka

ENTRYPOINT [ "./asuka" ]