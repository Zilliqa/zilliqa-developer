FROM golang:1.20.14 as build-stage
WORKDIR /app
COPY . /app
RUN go mod download && \
    CGO_ENABLED=0 GOOS=linux go build -o build/faucet-service ./cmd



FROM alpine:3.10 as final
WORKDIR /app
COPY --from=build-stage /app/build/faucet-service .
EXPOSE 8080
CMD ["./faucet-service"]
