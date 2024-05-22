<!-- trunk-ignore-all(markdownlint/MD029) -->

# Bridge Web

## How to run

```sh
npm install
npm run dev
```

## How to modify the encoded configuration

The steps to modify secrets are:

1. Set the variable
2. Decode the secret file

- staging

```sh
openssl aes-256-cbc -pbkdf2 -k "${ENV_FILES_DECRYPTER_NONPRD}" -in "./infra/environment/staging/.env.enc" -out "./infra/environment/staging/.env" -d
```

- production

```sh
openssl aes-256-cbc -pbkdf2 -k "${ENV_FILES_DECRYPTER_PRD}" -in "./infra/environment/production/.env.enc" -out "./infra/environment/production/.env" -d
```

3. Modify the decoded env file

```sh
openssl aes-256-cbc -pbkdf2 -k "${ENV_FILES_DECRYPTER_NONPRD}" -in "./infra/environment/staging/.env" -out "./infra/environment/staging/.env.enc"
```

- production

```sh
openssl aes-256-cbc -pbkdf2 -k "${ENV_FILES_DECRYPTER_PRD}" -in "./infra/environment/production/.env" -out "./infra/environment/production/.env.enc"
```

4. Encode the file
5. Submit the encoded file to the repo
