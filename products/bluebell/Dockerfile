FROM rustlang/rust:nightly AS builder
WORKDIR /src
ENV CARGO_HOME=/workdir/.cargo                       
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

COPY . /src/

RUN cd playground && NODE_ENV=production ../tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
RUN cargo +nightly build --release

FROM nginx:mainline-alpine3.18-slim
EXPOSE 80
COPY --from=builder /src/playground/dist/ /usr/share/nginx/html/
COPY --from=builder /src/nginx/default.conf /etc/nginx/conf.d/
