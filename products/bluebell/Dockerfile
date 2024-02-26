FROM ubuntu:22.04 AS builder
WORKDIR /src

RUN apt update && apt -y install curl build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN ~/.cargo/bin/rustup update

ENV CARGO_HOME="/root/.cargo/"
ENV PATH="/root/.cargo/bin/:${PATH}"
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
ENV PATH="/root/.cargo/bin:${PATH}"

# See https://github.com/cross-rs/cross/issues/260
RUN printf '#!/bin/sh\nexport CARGO_HOME=/opt/cargo\nexec /bin/sh "$@"\n' >/usr/local/bin/sh

RUN rustup target add wasm32-unknown-unknown

RUN apt-get update \
	&& apt-get install -y ca-certificates curl gnupg \
	&& mkdir -p /etc/apt/keyrings \
	&& curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg \
	&& NODE_MAJOR=20 \
	&& echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODE_MAJOR.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list \
	&& apt-get update \
	&& apt-get install nodejs -y \
	&& rm -rf /var/lib/apt/lists/*
RUN npm install -g tailwindcss
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
	&& chmod +x tailwindcss-linux-x64 \
	&& mv tailwindcss-linux-x64 tailwindcss

RUN cargo install --locked trunk

## Building

COPY . /src/
WORKDIR /src/playground/
ENV NODE_ENV=production
RUN ../tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
RUN trunk build


FROM nginx:mainline-alpine3.18-slim
EXPOSE 80
COPY --from=builder /src/playground/dist/ /usr/share/nginx/html/
COPY --from=builder /src/nginx/default.conf /etc/nginx/conf.d/
