# Git Integration Guide

This guide covers how to integrate WindWarden with Git workflows, pre-commit hooks, and CI/CD pipelines.

## Table of Contents

- [Pre-commit Hooks](#pre-commit-hooks)
- [Git Hooks](#git-hooks)
- [CI/CD Integration](#cicd-integration)
- [IDE Integration](#ide-integration)
- [Configuration](#configuration)

## Pre-commit Hooks

### Using pre-commit framework

The easiest way to integrate WindWarden with Git is using the [pre-commit](https://pre-commit.com/) framework.

#### 1. Install pre-commit

```bash
pip install pre-commit
# or
brew install pre-commit
```

#### 2. Create `.pre-commit-config.yaml`

```yaml
repos:
  - repo: local
    hooks:
      - id: windwarden
        name: WindWarden CSS Class Sorter
        entry: windwarden
        language: system
        files: \.(tsx?|jsx?)$
        args: ["--write"]
        pass_filenames: true
```

#### 3. Install the hook

```bash
pre-commit install
```

Now WindWarden will automatically sort your CSS classes before every commit!

### Advanced pre-commit configuration

For more control, you can specify additional arguments:

```yaml
repos:
  - repo: local
    hooks:
      - id: windwarden-check
        name: WindWarden Check (Verify Mode)
        entry: windwarden
        language: system
        files: \.(tsx?|jsx?)$
        args: ["--verify", "--format", "json"]
        pass_filenames: true
      - id: windwarden-format
        name: WindWarden Format
        entry: windwarden
        language: system
        files: \.(tsx?|jsx?)$
        args: ["--write", "--config", ".windwarden.json"]
        pass_filenames: true
```

## Git Hooks

### Manual Git hook setup

If you prefer not to use pre-commit, you can set up Git hooks manually.

#### Pre-commit hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/sh
# WindWarden pre-commit hook

# Check if windwarden is installed
if ! command -v windwarden &> /dev/null; then
    echo "Error: windwarden is not installed"
    exit 1
fi

# Get list of staged files
staged_files=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.(tsx?|jsx?)$')

if [ -z "$staged_files" ]; then
    exit 0
fi

echo "Running WindWarden on staged files..."

# Run windwarden in verify mode
if ! windwarden --verify $staged_files; then
    echo "❌ WindWarden found unsorted CSS classes"
    echo "Run 'windwarden --write $staged_files' to fix automatically"
    exit 1
fi

echo "✅ All CSS classes are properly sorted"
exit 0
```

Make it executable:

```bash
chmod +x .git/hooks/pre-commit
```

#### Pre-push hook

Create `.git/hooks/pre-push` for additional verification:

```bash
#!/bin/sh
# WindWarden pre-push hook

echo "Running final WindWarden check before push..."

# Check all tracked files
if ! windwarden --verify $(git ls-files | grep -E '\.(tsx?|jsx?)$'); then
    echo "❌ Some files have unsorted CSS classes"
    echo "Please run 'windwarden --write .' before pushing"
    exit 1
fi

echo "✅ All files pass WindWarden verification"
exit 0
```

Make it executable:

```bash
chmod +x .git/hooks/pre-push
```

## CI/CD Integration

### GitHub Actions

Create `.github/workflows/css-lint.yml`:

```yaml
name: CSS Class Linting

on:
  pull_request:
    paths:
      - '**/*.tsx'
      - '**/*.jsx'
      - '**/*.ts'
      - '**/*.js'

jobs:
  css-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install WindWarden
        run: |
          # See README.md for installation instructions
          # Using curl method for CI/CD
          curl -L https://github.com/benduggan/windwarden/releases/latest/download/windwarden-linux-x86_64 -o windwarden
          chmod +x windwarden
          sudo mv windwarden /usr/local/bin/
      
      - name: Check CSS class sorting
        run: |
          windwarden --verify --format json . > windwarden-results.json
          
      - name: Upload results
        if: failure()
        uses: actions/upload-artifact@v3
        with:
          name: windwarden-results
          path: windwarden-results.json
          
      - name: Comment PR
        if: failure()
        uses: actions/github-script@v6
        with:
          script: |
            const fs = require('fs');
            const results = JSON.parse(fs.readFileSync('windwarden-results.json', 'utf8'));
            
            let comment = '## WindWarden CSS Class Sorting Issues\n\n';
            comment += `Found ${results.warningCount} issues across ${results.results.length} files:\n\n`;
            
            for (const file of results.results) {
              if (file.messages.length > 0) {
                comment += `### ${file.filePath}\n`;
                for (const message of file.messages) {
                  comment += `- Line ${message.line}: ${message.message}\n`;
                }
                comment += '\n';
              }
            }
            
            comment += 'Run `windwarden --write .` to fix these issues automatically.';
            
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: comment
            });
```

### GitLab CI

Create `.gitlab-ci.yml`:

```yaml
stages:
  - lint

css-lint:
  stage: lint
  image: node:18-alpine
  before_script:
    - apk add --no-cache curl
    - curl -L https://github.com/your-org/windwarden/releases/latest/download/windwarden-linux -o windwarden
    - chmod +x windwarden
    - mv windwarden /usr/local/bin/
  script:
    - windwarden --verify --format json . > windwarden-results.json
  artifacts:
    when: on_failure
    reports:
      junit: windwarden-results.json
    expire_in: 1 week
  only:
    changes:
      - "**/*.tsx"
      - "**/*.jsx"
      - "**/*.ts"
      - "**/*.js"
```

### Azure DevOps

Create `azure-pipelines.yml`:

```yaml
trigger:
  branches:
    include:
      - main
      - develop
  paths:
    include:
      - "**/*.tsx"
      - "**/*.jsx"
      - "**/*.ts"
      - "**/*.js"

pool:
  vmImage: 'ubuntu-latest'

steps:
- script: |
    curl -L https://github.com/your-org/windwarden/releases/latest/download/windwarden-linux -o windwarden
    chmod +x windwarden
    sudo mv windwarden /usr/local/bin/
  displayName: 'Install WindWarden'

- script: |
    windwarden --verify --format json . > windwarden-results.json
  displayName: 'Check CSS class sorting'
  continueOnError: true

- task: PublishTestResults@2
  condition: always()
  inputs:
    testResultsFormat: 'JUnit'
    testResultsFiles: 'windwarden-results.json'
    testRunTitle: 'WindWarden CSS Lint Results'
```

## IDE Integration

### VS Code

WindWarden can integrate with VS Code through the command palette and tasks.

#### Create `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "WindWarden: Check Current File",
      "type": "shell",
      "command": "windwarden",
      "args": ["--verify", "${file}"],
      "group": "build",
      "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": false,
        "panel": "shared"
      },
      "problemMatcher": {
        "owner": "windwarden",
        "fileLocation": "absolute",
        "pattern": {
          "regexp": "^(.*):(\\d+):(\\d+):\\s+(warning|error)\\s+(.*)\\s+(.*)$",
          "file": 1,
          "line": 2,
          "column": 3,
          "severity": 4,
          "message": 5,
          "code": 6
        }
      }
    },
    {
      "label": "WindWarden: Format Current File",
      "type": "shell",
      "command": "windwarden",
      "args": ["--write", "${file}"],
      "group": "build"
    },
    {
      "label": "WindWarden: Format All Files",
      "type": "shell",
      "command": "windwarden",
      "args": ["--write", "."],
      "group": "build"
    }
  ]
}
```

#### Add keyboard shortcuts in `.vscode/keybindings.json`:

```json
[
  {
    "key": "ctrl+shift+w",
    "command": "workbench.action.tasks.runTask",
    "args": "WindWarden: Format Current File"
  }
]
```

### JetBrains IDEs (WebStorm, IntelliJ)

1. Go to `File > Settings > Tools > External Tools`
2. Click `+` to add a new tool
3. Configure:
   - **Name**: WindWarden Check
   - **Program**: `windwarden`
   - **Arguments**: `--verify $FilePath$`
   - **Working directory**: `$ProjectFileDir$`

4. Add another tool for formatting:
   - **Name**: WindWarden Format
   - **Program**: `windwarden`
   - **Arguments**: `--write $FilePath$`
   - **Working directory**: `$ProjectFileDir$`

## Configuration

### Repository-level configuration

Create `.windwarden.json` in your project root:

```json
{
  "sortOrder": "official",
  "functionNames": ["cn", "clsx", "twMerge", "classnames"],
  "ignorePatterns": [
    "node_modules/**",
    "dist/**",
    "build/**"
  ],
  "fileExtensions": ["tsx", "jsx", "ts", "js"],
  "git": {
    "respectGitignore": true,
    "onlyGitFiles": false
  },
  "safety": {
    "atomicWrites": true,
    "createBackups": false
  }
}
```

### Team workflow recommendations

1. **Add configuration to version control**: Include `.windwarden.json` in your repository
2. **Document in README**: Add setup instructions for new team members
3. **Use in CI**: Fail builds on unsorted classes to maintain consistency
4. **Gradual adoption**: Start with `--verify` mode, then enable `--write` after team training

### Git ignore patterns

Add to `.gitignore` if using backup features:

```gitignore
# WindWarden backups
*.bak
*.bak.*
```

## Troubleshooting

### Common issues

#### Pre-commit hook fails
```bash
# Check if windwarden is in PATH
which windwarden

# Check version
windwarden --version

# Test on a single file
windwarden --verify src/components/Button.tsx
```

#### CI/CD fails to install
- Check release URL and platform (linux/mac/windows)
- Verify binary permissions
- Consider using package managers (npm, cargo, etc.)

#### Large repositories are slow
- Use `.windwardenignore` to exclude unnecessary files
- Run only on changed files in CI
- Use parallel processing with `--threads` option

### Getting help

- Check the main README for basic usage
- Use `windwarden --help` for command reference
- Report issues on the GitHub repository
- Join the community discussions

## Best Practices

1. **Start small**: Begin with a single directory or component
2. **Train the team**: Ensure everyone understands the sorting rules
3. **Use automation**: Let tools handle the formatting automatically
4. **Be consistent**: Use the same configuration across all environments
5. **Monitor**: Use CI/CD to catch issues before they reach main branch

## Examples

### Example workflow for new features

```bash
# 1. Start feature branch
git checkout -b feature/new-component

# 2. Write your component with any class order
# 3. Before committing, format with WindWarden
windwarden --write src/components/NewComponent.tsx

# 4. Commit changes
git add .
git commit -m "Add new component"

# 5. Push - CI will verify everything is sorted
git push origin feature/new-component
```

### Example team onboarding

```bash
# 1. Clone repository
git clone https://github.com/team/project.git
cd project

# 2. Install dependencies including WindWarden
npm install  # or your package manager

# 3. Install pre-commit hooks
pre-commit install

# 4. Format existing files (one-time setup)
windwarden --write .

# 5. You're ready to go!
```

This integration ensures consistent CSS class sorting across your entire team and codebase!