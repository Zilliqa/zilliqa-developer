#!/bin/bash

mv .env_stg .env
npm install
npm rebuild node-sass
npm run build