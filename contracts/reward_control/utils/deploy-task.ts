import * as fs from "fs";
import * as path from "path";
import { task } from "hardhat/config";

task("deploy", "Deploy stuff")
  .addOptionalParam("only")
  .setAction(async (taskArgs) => {
    let only = taskArgs.only;
    let theRegex = undefined;
    if (only !== undefined) {
      theRegex = `^${only}$`;
    }
    // Find the deployment scripts.
    let deployContents = fs.readdirSync(path.resolve(process.cwd(), "deploy"), {
      encoding: "utf8",
    });
    deployContents = deployContents
      .filter((filename) => {
        return filename.endsWith(".ts");
      })
      .sort();
    for (let deployFile of deployContents) {
      if (only == undefined || deployFile.match(theRegex)) {
        console.log(`> ${deployFile}`);
        let f = await import(path.resolve(process.cwd(), "deploy", deployFile));
        await f.default();
        console.log(`Done`);
      }
    }
    console.log(`${JSON.stringify(deployContents)}`);
    console.log(taskArgs);
  });
