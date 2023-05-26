# Example: Get latest block

## Quickstart: Use boilerplate project

1. Make sure that the library was build. From the repository root go to
   `zilliqa/js`. Then run

   ```sh
   pnpm i -r
   pnpm -r build
   ```

2. Change directory to the `latest-block` directory and install the
   dependencies:

   ```sh
   pnpm i
   ```

3. Build and run the script:

   ```sh
   npx tsc
   node dist/index.js
   ```

## Project from scratch

1. Create a new directory to work in

   ```sh
   mkdir zilliqa-latest-block
   cd  zilliqa-latest-block
   ```

2. Initialise the directory with npm and typescript

   ```sh
   npm init
   npm install --save-dev typescript @types/node
   npx tsc --init
   ```

3. Update `tsconfig.json`. It should look like:

   ```json
   {
     "compilerOptions": {
       "target": "es5",
       "module": "commonjs",
       "outDir": "dist",
       "strict": true,
       "esModuleInterop": true
     },
     "include": ["src/**/*.ts"],
     "exclude": ["node_modules"]
   }
   ```

4. Install the `zilliqa` libraries

   ```sh
   npm install @zilliqa-js/zilliqa @zilliqa-js/crypto @zilliqa-js/util @zilliqa-js/core @zilliqa-js/account @zilliqa-js/contract @zilliqa-js/blockchain
   ```

5. Create the source directory and `index.ts`:

   ```sh
   mkdir src
   touch src/index.ts
   ```

6. Update `index.ts` with:

   ```ts
   import { Zilliqa } from "@zilliqa-js/zilliqa";

   async function main() {
     const provider = new Zilliqa("https://api.zilliqa.com/");
     const latestBlock = await provider.blockchain.getLatestTxBlock();
     console.log(latestBlock);
   }
   main();
   ```

7. Compile and run:

   ```sh
   npx tsc
   node dist/index.js
   ```
