FROM debian:12.11

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential libpq-dev -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY ./ /app
RUN make prod

FROM debian:12.11

ENV DEBIAN_FRONTEND=noninteractive

ARG TOKEN

ENV DTOKEN=$TOKEN

RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential libpq-dev uuid-runtime -y

RUN uuidgen > /etc/machine-id

WORKDIR /app

RUN mkdir -p /app/assets

COPY --from=0 /app/Settings.toml /app
COPY --from=0 /app/assets/jueves.gif /app/assets
COPY --from=0 /app/target/release/asuka /app

RUN echo $DTOKEN | ./asuka

ENTRYPOINT [ "./asuka" ]