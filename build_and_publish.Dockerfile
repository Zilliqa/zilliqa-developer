# Use the official Node.js 18 image as the base
FROM node:18

# Set the working directory inside the container
WORKDIR /app

# Install Python and build tools required for native dependencies
RUN apt-get update && apt-get install -y python3 build-essential

# Install pnpm version 8.x globally, which is compatible with lockfileVersion 6.0
RUN npm install -g pnpm@8

# Copy package.json and pnpm-lock.yaml to the working directory
COPY pnpm-lock.yaml .
COPY tsconfig.base.json .
COPY package.json .

# Generate the pnpm-workspace.yaml file with the specified content
RUN echo 'packages:\n  - "zilliqa/js/account"\n  - "zilliqa/js/blockchain"\n  - "zilliqa/js/contract"\n  - "zilliqa/js/core"\n  - "zilliqa/js/crypto"\n  - "zilliqa/js/proto"\n  - "zilliqa/js/subscriptions"\n  - "zilliqa/js/typings"\n  - "zilliqa/js/util"\n  - "zilliqa/js/zilliqa"' > pnpm-workspace.yaml

COPY zilliqa/js zilliqa/js/

# Install project dependencies.
RUN pnpm -r install --frozen-lockfile

# Build the project
ENV NODE_OPTIONS=--max-old-space-size=4096
RUN for pkg in zilliqa/js/typings zilliqa/js/core zilliqa/js/proto zilliqa/js/util zilliqa/js/crypto zilliqa/js/account zilliqa/js/blockchain zilliqa/js/contract zilliqa/js/subscriptions zilliqa/js/zilliqa; do echo "Building $pkg..."; pnpm --dir "$pkg" build; done

# See Readme > Building and publishing `zilliqa-js` for the complete publishing process.
CMD ["echo", "1"]
