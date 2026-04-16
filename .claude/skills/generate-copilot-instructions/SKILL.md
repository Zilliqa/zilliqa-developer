---
name: generate-copilot-instructions
description: Generate `.github/copilot-instructions.md` based on the `CLAUDE.md`
---

Translate the project-specific rules and coding standards from `CLAUDE.md` into a format optimized for GitHub Copilot.

## Usage
- When requested to sync or generate Copilot instructions.
- After significant updates to `CLAUDE.md`.

## Instructions

1. **Locate Source**: Read the `CLAUDE.md` file in the project root.
2. **Identify Target**: Prepare to write to `.github/copilot-instructions.md`.
3. **Analyze Content**:
    - Extract **Coding Standards**: Language-specific rules, naming conventions, and formatting.
    - Extract **Project Architecture**: Folder structures, design patterns, and tech stack details.
    - Extract **Documentation Requirements**: JSDoc, TypeDoc, or README update rules.
    - **Filter Out**: Remove agent-specific terminal commands (like "Build command: npm run build") as Copilot handles these differently than agentic tools like Cline.
4. **Transform**:
    - Reformat the instructions into a clear, hierarchical Markdown structure.
    - Use imperative language (e.g., "Always use...", "Avoid...", "Ensure...").
    - Group instructions into logical sections: `Code Style`, `Architecture`, `Testing`, and `Patterns`.
5. **Execution**:
    - Create the `.github` directory if it does not exist.
    - Write the refined content to `.github/copilot-instructions.md`.
    - Provide a brief summary of the changes made to the user.