#!/usr/bin/env node

import * as inquirer from "inquirer";
import * as fs from "fs";
import * as path from "path";
import * as shell from "shelljs";
import * as template from "../utils/template";
import * as chalk from "chalk";
import * as yargs from "yargs";

const CHOICES = fs.readdirSync(path.join(__dirname, "../templates"));
const SKIP_FILES = ["node_modules", ".template.json"];
const CURR_DIR = process.cwd();

const QUESTIONS = [
  {
    name: "template",
    type: "list",
    message: "What contract template would you like to generate?",
    choices: CHOICES,
    when: () => !yargs.argv["template"],
  },
  {
    name: "name",
    type: "input",
    message: "Project name:",
    when: () => !yargs.argv["name"],
    validate: (input: string) => {
      if (/^([A-Za-z\-\_\d])+$/.test(input)) return true;
      else
        return "Project name may only include letters, numbers, underscores and hashes.";
    },
  },
];

export interface TemplateConfig {
  files?: string[];
  postMessage?: string;
}

export interface CliOptions {
  projectName: string;
  templateName: string;
  templatePath: string;
  tartgetPath: string;
  config: TemplateConfig;
}

function showMessage(options: CliOptions) {
  console.log("");
  console.log(chalk.green("Scaffoling done."));
  console.log(
    "Enter project directory: ",
    chalk.green(`cd ${options.projectName}`)
  );
  const message = options.config.postMessage;

  if (message) {
    console.log("");
    console.log(chalk.yellow(message));
    console.log("");
  }
}

function getTemplateConfig(templatePath: string): TemplateConfig {
  const configPath = path.join(templatePath, ".template.json");

  if (!fs.existsSync(configPath)) return {};

  const templateConfigContent = fs.readFileSync(configPath);

  if (templateConfigContent) {
    return JSON.parse(templateConfigContent.toString());
  }

  return {};
}

function createProject(projectPath: string) {
  if (fs.existsSync(projectPath)) {
    console.log(
      chalk.red(`Folder ${projectPath} exists. Delete or use another name.`)
    );
    return false;
  }

  fs.mkdirSync(projectPath);
  return true;
}

function postProcess(options: CliOptions) {
  if (isNode(options)) {
    return postProcessNode(options);
  }
  return true;
}

function isNode(options: CliOptions) {
  return fs.existsSync(path.join(options.templatePath, "package.json"));
}

function postProcessNode(options: CliOptions) {
  shell.cd(options.tartgetPath);

  let cmd = "";

  if (shell.which("yarn")) {
    cmd = "yarn";
  } else if (shell.which("npm")) {
    cmd = "npm install";
  }

  if (cmd) {
    const result = shell.exec(cmd);

    if (result.code !== 0) {
      return false;
    }
  } else {
    console.log(chalk.red("No yarn or npm found. Cannot run installation."));
  }

  return true;
}

function createDirectoryContents(
  templatePath: string,
  projectName: string,
  projectChoice: string,
  config: TemplateConfig
) {
  const filesToCreate = fs.readdirSync(templatePath);

  filesToCreate.forEach((file) => {
    const origFilePath = path.join(templatePath, file);

    // get stats about the current file
    const stats = fs.statSync(origFilePath);

    if (SKIP_FILES.indexOf(file) > -1) return;

    if (stats.isFile()) {
      let contents = fs.readFileSync(origFilePath, "utf8");

      try {
        if (projectChoice === "blank-project") {
          const rarr = projectName.split("/");
          if (rarr.length) {
            contents = template.render(contents, { projectName: rarr[0] });
          } else {
            contents = template.render(contents, { projectName });
          }
        }
      } catch (error) {
        console.log(file);
        console.error(error);
      }

      if (file === "contract.scilla") {
        file = `${projectName}.scilla`;
      }

      const writePath = path.join(CURR_DIR, projectName, file);
      fs.writeFileSync(writePath, contents, "utf8");
    } else if (stats.isDirectory()) {
      fs.mkdirSync(path.join(CURR_DIR, projectName, file));

      // recursive call
      createDirectoryContents(
        path.join(templatePath, file),
        path.join(projectName, file),
        projectChoice,
        config
      );
    }
  });
}

const init = () => {
  return inquirer
    .prompt(QUESTIONS)
    .then((answers: { template: string; name: string }) => {
      answers = Object.assign({}, answers, yargs.argv);

      const projectChoice = answers.template;
      const projectName = answers.name;
      const templatePath = path.join(__dirname, "../templates", projectChoice);
      const tartgetPath = path.join(CURR_DIR, projectName);
      const templateConfig = getTemplateConfig(templatePath);

      const options: CliOptions = {
        projectName,
        templateName: projectChoice,
        templatePath,
        tartgetPath,
        config: templateConfig,
      };

      if (!createProject(tartgetPath)) {
        return;
      }

      createDirectoryContents(
        templatePath,
        projectName,
        projectChoice,
        templateConfig
      );

      if (!postProcess(options)) {
        return;
      }

      showMessage(options);
    });
};

export default init;
