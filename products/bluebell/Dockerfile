FROM rustlang/rust:nightly AS builder
WORKDIR /src
ENV CARGO_HOME=/workdir/.cargo                       
ADD . /src/
RUN NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
RUN cargo +nightly build --release

FROM nginx:mainline-alpine3.18-slim
EXPOSE 80
COPY --from=builder /src/playground/dist/ /usr/share/nginx/html/
COPY --from=builder /src/nginx/default.conf /etc/nginx/conf.d/
