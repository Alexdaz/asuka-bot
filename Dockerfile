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

RUN apt-get update && \
    apt-get install -y libcurl4 libssl3 uuid-runtime && \
    touch .asukadocker && \
    uuidgen > /etc/machine-id && \
    apt-get purge -y --auto-remove uuid-runtime && \
    rm -rf /var/lib/apt/lists/*

RUN adduser --disabled-password --gecos "" asuka

RUN mkdir -p /app

COPY --chown=asuka:asuka --from=builder /app/Settings.toml /app
COPY --chown=asuka:asuka --from=builder /app/assets /app/assets
COPY --chown=asuka:asuka --from=builder /app/target/release/asuka /app/asuka

RUN echo $DTOKEN | /app/asuka

RUN mv .env.ask /app && chown -R asuka:asuka /app

WORKDIR /app

RUN rm -f /.asukadocker

USER asuka

ENTRYPOINT ["/app/asuka"]
