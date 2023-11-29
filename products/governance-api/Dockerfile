FROM node:14.17.3

WORKDIR /app
COPY . .
RUN npm install

EXPOSE 3000

ARG SCRIPT="start"
ENV SCRIPT=${SCRIPT}
ENTRYPOINT ["bash", "run.sh"]
