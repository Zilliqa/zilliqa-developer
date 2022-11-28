# Contributing Guideline

Help us to make this a better place for documentaion.

- [Markdown](#markdown)
- [Workflow](#workflow)
- [Styling](#Styling)

## Markdown

The entire repository accepts only documention in markdown format. It's lightweight styling language for easier writing, editing and reading if you enjoy the rendered version.

Not faimiliar with it? No worries, [master markdown in 3 minutes](https://guides.github.com/features/mastering-markdown/).

If you haven't found a handy tool for markdown editing and previewing, check [this doc](https://code.visualstudio.com/docs/languages/markdown) from VSCode.

## Workflow

For small revisions, you can directly [edit the markdown files on Github](https://help.github.com/en/articles/editing-files-in-your-repository) and create a pull-request from there.

For major changes, it takes a few more steps.

1. Clone the repository locally.
2. Checkout a new branch and commit your changes.
3. Create a pull request on Github.
4. Invite reviewers to proof-read and suggest improvement.
5. Finalize the pull request and have it merged to master.

**Rebase and merge**: We will [rebase and merge](https://help.github.com/en/articles/about-pull-request-merges#rebase-and-merge-your-pull-request-commits) the pull-request once it's good to go. You may ask to resolve the conflicts before it can be merged.

## Styling

Although the markdown docs will be rendered nicely for the readers, the source file style also matters. The agreement on the styling is not easy to make so we simply start with [markdownlint](https://github.com/markdownlint/markdownlint) and its [rules](https://github.com/markdownlint/markdownlint/blob/master/docs/RULES.md).

### Configuring the rules

The universal configuration file is `.markdownlint.yaml`. Feel free to propose changes in this file if you feel rules are too strict.

To temporaily disable some rules, you can prepend inline HTML comment in the markdown file

```html
<!-- markdownlint-disable MD033 MD041 MD002 -->
```

Check [this](https://github.com/DavidAnson/markdownlint#optionsconfig) to find more details about configuring the rules.

### Linting

Style checking is automatically done on Travis after every commit and it is required to pass before the pull request can be merged. See `.travis.yml` for more details.

On your machine, you can install `markdownlint-cli`

```bash
npm install -g markdownlint-cli
```

and simply run from the project root to check if you have any style issues.

```bash
markdownlint *.md
```

You can add this command as a pre-commit git hook. Add the following content to `.git/hook/pre-commit` and make sure it's executable (`chmod +x pre-commit`)

```bash
#!/bin/bash
markdownlint *.md
```

There's also a great VSCode extension, [markdownlint](https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint), that does all the checks for you while you are editing the file. It respects the rules configured for this repository.
