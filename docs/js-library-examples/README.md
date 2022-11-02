# Zilliqa-JavaScript-Library-Examples

This repository contains some examples on how to use [Zilliqa Javascript SDK](https://github.com/Zilliqa/Zilliqa-JavaScript-Library). Zilliqa Javascript can be used in two approaches; the common NodeJS approach and a traditional HTML Javascript approach. 


## NodeJS (Recommended)
The NodeJS approach is to add `@zilliqa-js/zilliqa` module from https://www.npmjs.com/package/@zilliqa-js/zilliqa. To use Zilliqa SDK in the project, users must `require` it in their frontend frameworks.
```
yarn add `@zilliqa-js/zilliqa`

// index.js
const { Zilliqa } = require('@zilliqa-js/zilliqa');
const zilliqa = new Zilliqa('https://dev-api.zilliqa.com');
```

See `node` for detailed examples.

**This is the recommended way for frontend JS frameworks such as React, Angular, VueJS, etc.**


## HTML script (Alternative)
Zilliqa Javascript can also be build to a `zilliqa.min.js` which allows users to import them into a HTML with the `<script>` tag. One caveat is that users would need to build the minified JS file from the main project themselves.

```
git clone https://github.com/Zilliqa/Zilliqa-JavaScript-Library
cd Zilliqa-JavaScript-Library
yarn install
yarn build:web

// zilliqa.min.js is generated under Zilliqa-JavaScript-Library/dist
cp Zilliqa-JavaScript-Library/dist/zilliqa.min.js /webpack/react/public

cd webpack/react
yarn install
yarn build:web
```

See `webpack/html` and `webpack/react` for detailed examples.

**Frontend developers should generally use the NodeJS approach to use Zilliqa SDK unless there are some constraints.**