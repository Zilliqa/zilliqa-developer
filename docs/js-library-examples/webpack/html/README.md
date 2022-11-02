# HTMl script Example

This example will demonstrate how to use the **webpack variant** of ZilliqaJS in a ordinary HTML file.
The webpack variant comes in a `zilliqa.min.js` file and is meant for users who wishes to use ZilliqaJS functions in traditional HTML. Users need to build the webpack variant and import the js file in the HTML.

## Note: Frontend Framework Users
If you are using a frontend framework such as React, Vue, AngularJS, etc, we **strongly recommend you to use our node version** instead of the webpack variant as the schematic is easier to work with. Skip to the [Zilliqa-Javascript-Library README](https://github.com/Zilliqa/Zilliqa-JavaScript-Library#installation) for the NodeJS approach examples. If you are still interested in using the webpack variant, continue below.

## How to get started

1. Build the webpack variant to generate a `zilliqa.min.js` located under `Zilliqa-JavaScript-Library/dist`.
```
git clone https://github.com/Zilliqa/Zilliqa-JavaScript-Library
cd Zilliqa-JavaScript-Library
yarn install
yarn build:web
```

2. Create a `index.html`.
```
cd webpack/html

// index.html

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
    <!-- Please generate zilliqa-min.js using yarn build:web-->>
    <script src="zilliqa.min.js"></script>
    <script src="index.js"></script>
    <script src="load-keystore-demo.js"></script>
</head>
<body>
</body>
</html>

```

The three script tags performs the following:
```
    <script src="zilliqa.min.js"></script>    // import the minified JS we built from (1)
    <script src="index.js"></script> // usage example 1
    <script src="load-keystore-demo.js"></script> // usage example 2
```


2. Copy `zilliqa.min.js` into the folder with index.html.
```
cp Zilliqa-JavaScript-Library/dist/zilliqa.min.js /webpack/html

Double click on HTML file

Open the dev console, and should see some transaction statements. The `index.js` and `load-keystore-demo.js` show how to send a payment transaction and export a keystore file respectively.
```