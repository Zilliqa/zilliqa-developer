FROM node:20

WORKDIR /app

COPY package.json yarn.lock ./
RUN npm install

COPY . .

RUN groupadd -r app \
    && useradd -r -g app app \
    && chown -R app:app /app

USER app

ENV SCRIPT=start

EXPOSE 3000

ENTRYPOINT ["bash", "run.sh"]
