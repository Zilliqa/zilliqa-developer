// Utilities for reading configuration files.
import * as YAML from "yaml";
import * as fs from "fs";

export enum ContractType {
  EVM,
  Scilla,
}

export interface ContractDescription {
  name: string;
  chainId: Number;
  address: string;
  extra: any;
  contractType: ContractType;
}

export interface Account {
  privkey: string;
}

// function getConfigFilename(relname: string): string {
//   const fileName = `${process.env.Z_CONFIG}/${relname}.yaml`;
//   return fileName;
// }

// function readYaml(relname: string): any {
//   const fileName = getConfigFilename(relname);
//   const contents = fs.readFileSync(fileName, "utf-8");
//   const parsed = YAML.parse(contents);
//   return parsed;
// }

export function getDeploymentDescription(
  forContract: string
): ContractDescription | undefined {
  let fileName = getDeploymentFilename(forContract);
  if (!fs.existsSync(fileName)) {
    return undefined;
  }
  let someData = fs.readFileSync(fileName, { encoding: "utf8" });
  let result = YAML.parse(someData) as ContractDescription;
  return result;
}

// export function getSecrets(forContract: string): any {
//   let fileName = getSecretsFilename(forContract);
//   let someData = fs.readFileSync(fileName, { encoding: "utf8" });
//   if (someData === undefined) {
//     return undefined;
//   }
//   let result = YAML.parse(someData);
//   return result;
// }

// export function updateDeploymentDescription(description: ContractDescription) {
//   let fileName = getDeploymentFilename(description.name);
//   fs.writeFileSync(fileName, YAML.stringify(description), { mode: 0o644 });
// }

// export function updateSecrets(name: string, secrets: any) {
//   let fileName = getSecretsFilename(name);
//   fs.writeFileSync(fileName, YAML.stringify(secrets), { mode: 0o644 });
// }

// export function getDeploymentFilename(forContract: string): string {
//   let directory = `${process.env.Z_CONFIG}/deployments`;
//   if (!fs.existsSync(directory)) {
//     fs.mkdirSync(directory, { recursive: true, mode: 0o755 });
//   }
//   return `${directory}/${forContract}.yaml`;
// }

// export function getSecretsFilename(forContract: string): string {
//   let directory = `${process.env.Z_CONFIG}/secrets`;
//   if (!fs.existsSync(directory)) {
//     fs.mkdirSync(directory, { recursive: true, mode: 0o755 });
//   }
//   return `${directory}/${forContract}.yaml`;
// }

// export function baseConfig(): any {
//   return readYaml("base");
// }

// // Turn a list of .-separated key/value pairs into an object heirarchy
// export function assembleObjects(flat_map: any): any {
//   let result = {};
//   for (let k in flat_map) {
//     let v = flat_map[k];
//     let entries = k.split(".");
//     let cur = result;
//     for (let e of entries.slice(0, -1)) {
//       let next = {};
//       if (e in cur) {
//         next = cur[e];
//       } else {
//         cur[e] = next;
//       }
//       cur = next;
//     }
//     let final = entries.pop();
//     cur[final] = v;
//   }
//   return result;
// }

// export function baseSecrets(): any {
//   if (!fs.existsSync(getConfigFilename("secrets.cache"))) {
//     throw Error(
//       "secrets.cache.yaml does not exist - run z dev compile-secrets"
//     );
//   }
//   let keys = readYaml("secrets.cache");
//   let obj = assembleObjects(keys);
//   return obj;
// }

export function getAccounts(): Object {
  return baseSecrets().accounts;
}

export function getPrivKeys(): string[] {
  let m = Object.values(getAccounts()).map((x) => x.privkey);
  return m;
}

export function getEthChainId(): Number {
  let chainId = baseConfig().chainId;
  chainId = chainId | 0x8000;
  return chainId;
}

export function getZilliqaChainId(): Number {
  let chainId = baseConfig().chainId;
  chainId = chainId & ~0x8000;
  return chainId;
}
