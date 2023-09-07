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
COPY . /src/

# RUN NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
RUN cargo +nightly build --release

FROM nginx:mainline-alpine3.18-slim
EXPOSE 80
COPY --from=builder /src/playground/dist/ /usr/share/nginx/html/
COPY --from=builder /src/nginx/default.conf /etc/nginx/conf.d/
