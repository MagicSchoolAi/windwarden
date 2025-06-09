# Shell Completions for WindWarden

WindWarden provides tab completion support for bash, zsh, fish, and PowerShell. This guide covers how to install and use completions for each shell.

## Table of Contents

- [Quick Install](#quick-install)
- [Bash](#bash)
- [Zsh](#zsh)
- [Fish](#fish)
- [PowerShell](#powershell)
- [Troubleshooting](#troubleshooting)
- [Features](#features)

## Quick Install

WindWarden can generate completion scripts for your shell:

```bash
# Generate completions for your shell
windwarden completions bash    # For bash
windwarden completions zsh     # For zsh  
windwarden completions fish    # For fish
windwarden completions powershell  # For PowerShell
```

## Bash

### System-wide Installation (recommended)

```bash
# Generate and install completions system-wide
windwarden completions bash | sudo tee /etc/bash_completion.d/windwarden

# Or for newer systems using bash-completion v2
windwarden completions bash | sudo tee /usr/share/bash-completion/completions/windwarden
```

### User-specific Installation

```bash
# Create completions directory if it doesn't exist
mkdir -p ~/.local/share/bash-completion/completions

# Install completion script
windwarden completions bash > ~/.local/share/bash-completion/completions/windwarden

# Add to your ~/.bashrc if not already present
echo 'source ~/.local/share/bash-completion/completions/windwarden' >> ~/.bashrc
```

### Manual Installation

Add this to your `~/.bashrc`:

```bash
# WindWarden completions
eval "$(windwarden completions bash)"
```

### Testing

```bash
# Reload your shell or source bashrc
source ~/.bashrc

# Test completions (type this and press TAB)
windwarden <TAB>
windwarden format --<TAB>
windwarden config <TAB>
```

## Zsh

### Oh My Zsh Installation

```bash
# Create completions directory
mkdir -p ~/.oh-my-zsh/completions

# Install completion script
windwarden completions zsh > ~/.oh-my-zsh/completions/_windwarden

# Add to your ~/.zshrc if not already present
echo 'fpath=(~/.oh-my-zsh/completions $fpath)' >> ~/.zshrc
echo 'autoload -U compinit && compinit' >> ~/.zshrc
```

### System-wide Installation

```bash
# Install to system completions directory
windwarden completions zsh | sudo tee /usr/local/share/zsh/site-functions/_windwarden

# Or on some systems:
windwarden completions zsh | sudo tee /usr/share/zsh/vendor-completions/_windwarden
```

### User-specific Installation

```bash
# Create user completions directory
mkdir -p ~/.zsh/completions

# Install completion script
windwarden completions zsh > ~/.zsh/completions/_windwarden

# Add to your ~/.zshrc
echo 'fpath=(~/.zsh/completions $fpath)' >> ~/.zshrc
echo 'autoload -U compinit && compinit' >> ~/.zshrc
```

### Manual Installation

Add this to your `~/.zshrc`:

```bash
# WindWarden completions
eval "$(windwarden completions zsh)"
```

### Testing

```bash
# Reload your shell
exec zsh

# Test completions (type this and press TAB)
windwarden <TAB>
windwarden format --mode <TAB>
windwarden completions <TAB>
```

## Fish

### Installation

```bash
# Create completions directory
mkdir -p ~/.config/fish/completions

# Install completion script
windwarden completions fish > ~/.config/fish/completions/windwarden.fish
```

### System-wide Installation

```bash
# Install system-wide (usually requires sudo)
windwarden completions fish | sudo tee /usr/share/fish/vendor_completions.d/windwarden.fish
```

### Testing

```bash
# Reload fish configuration
exec fish

# Test completions (type this and press TAB)
windwarden <TAB>
windwarden format --<TAB>
windwarden config <TAB>
```

## PowerShell

### Windows PowerShell / PowerShell Core

```powershell
# Create profile directory if it doesn't exist
if (!(Test-Path -Path $PROFILE)) {
    New-Item -ItemType File -Path $PROFILE -Force
}

# Add WindWarden completions to your profile
Add-Content -Path $PROFILE -Value "windwarden completions powershell | Out-String | Invoke-Expression"
```

### Alternative Installation

```powershell
# Create a dedicated completions directory
$completionsDir = "$env:USERPROFILE\Documents\PowerShell\Completions"
New-Item -ItemType Directory -Path $completionsDir -Force

# Generate completion script
windwarden completions powershell > "$completionsDir\windwarden.ps1"

# Add to your profile
Add-Content -Path $PROFILE -Value ". '$completionsDir\windwarden.ps1'"
```

### Testing

```powershell
# Reload PowerShell profile
. $PROFILE

# Test completions (type this and press TAB)
windwarden <TAB>
windwarden format --<TAB>
windwarden config <TAB>
```

## Troubleshooting

### Completions Not Working

#### Check Installation

```bash
# Verify windwarden is in PATH
which windwarden

# Test completion generation
windwarden completions bash --help
```

#### Check Shell Configuration

**Bash:**
```bash
# Check if bash-completion is installed
ls /etc/bash_completion.d/
ls /usr/share/bash-completion/completions/

# Check if completion is loaded
complete -p windwarden
```

**Zsh:**
```bash
# Check fpath
echo $fpath

# Check if completion function exists
which _windwarden
```

**Fish:**
```bash
# Check completions directory
ls ~/.config/fish/completions/

# Test completion directly
complete -C windwarden
```

### Common Issues

#### Issue: "Command not found"

**Solution:** Ensure WindWarden is installed and in your PATH:

```bash
# Check if windwarden is accessible
windwarden --version

# If not, add to PATH or use full path
export PATH="/path/to/windwarden:$PATH"
```

#### Issue: "Completions not updating"

**Solution:** Regenerate and reinstall completions:

```bash
# Remove old completions
rm ~/.local/share/bash-completion/completions/windwarden

# Reinstall with latest version
windwarden completions bash > ~/.local/share/bash-completion/completions/windwarden

# Restart shell
exec bash
```

#### Issue: "Permission denied"

**Solution:** Use appropriate permissions:

```bash
# For user-specific installation
chmod +r ~/.local/share/bash-completion/completions/windwarden

# For system-wide installation
sudo windwarden completions bash > /etc/bash_completion.d/windwarden
```

#### Issue: "Slow completions"

**Solution:** Some shells cache completions. Clear cache or restart shell:

```bash
# Zsh: Clear completion cache
rm ~/.zcompdump*
exec zsh

# Fish: Clear completion cache
fish -c "complete -e"
exec fish
```

### Manual Debugging

#### Test Raw Completion

```bash
# Generate completion script and check for errors
windwarden completions bash > /tmp/test-completion.bash
source /tmp/test-completion.bash

# Test specific completion
complete -F _windwarden windwarden
```

#### Check Environment

```bash
# Check shell version
echo $BASH_VERSION  # For bash
echo $ZSH_VERSION   # For zsh
fish --version      # For fish

# Check if completion system is enabled
shopt -s progcomp  # For bash (should show nothing if enabled)
```

## Features

WindWarden's shell completions provide smart tab completion for:

### Commands and Subcommands

```bash
windwarden <TAB>
# Shows: format, check, config, completions

windwarden config <TAB>  
# Shows: init, show, validate
```

### Options and Flags

```bash
windwarden format --<TAB>
# Shows: mode, processing, threads, extensions, exclude, max-depth, etc.

windwarden format --mode <TAB>
# Shows: check, write, verify
```

### File and Directory Paths

```bash
windwarden format src/<TAB>
# Shows: files and directories in src/

windwarden config validate <TAB>
# Shows: available config files
```

### Configuration Values

```bash
windwarden format --extensions <TAB>
# Shows: common file extensions (tsx, jsx, ts, js)

windwarden completions <TAB>
# Shows: bash, zsh, fish, powershell
```

### Context-Aware Completions

The completions are context-aware and only show relevant options:

```bash
windwarden format --mode write --<TAB>
# Only shows options compatible with write mode

windwarden config init --<TAB>
# Shows: path (since that's the only option for init)
```

## Installation Scripts

### Automatic Installation Script (Bash/Zsh)

Create this script for easy installation:

```bash
#!/bin/bash
# install-windwarden-completions.sh

set -e

SHELL_NAME=$(basename "$SHELL")

case "$SHELL_NAME" in
    bash)
        if command -v windwarden >/dev/null 2>&1; then
            echo "Installing bash completions..."
            mkdir -p ~/.local/share/bash-completion/completions
            windwarden completions bash > ~/.local/share/bash-completion/completions/windwarden
            echo "✓ Bash completions installed to ~/.local/share/bash-completion/completions/windwarden"
            echo "Please restart your shell or run: source ~/.bashrc"
        else
            echo "Error: windwarden not found in PATH"
            exit 1
        fi
        ;;
    zsh)
        if command -v windwarden >/dev/null 2>&1; then
            echo "Installing zsh completions..."
            mkdir -p ~/.zsh/completions
            windwarden completions zsh > ~/.zsh/completions/_windwarden
            echo "✓ Zsh completions installed to ~/.zsh/completions/_windwarden"
            echo "Add this to your ~/.zshrc if not already present:"
            echo "  fpath=(~/.zsh/completions \$fpath)"
            echo "  autoload -U compinit && compinit"
        else
            echo "Error: windwarden not found in PATH"
            exit 1
        fi
        ;;
    fish)
        if command -v windwarden >/dev/null 2>&1; then
            echo "Installing fish completions..."
            mkdir -p ~/.config/fish/completions
            windwarden completions fish > ~/.config/fish/completions/windwarden.fish
            echo "✓ Fish completions installed to ~/.config/fish/completions/windwarden.fish"
            echo "Please restart fish or run: exec fish"
        else
            echo "Error: windwarden not found in PATH"
            exit 1
        fi
        ;;
    *)
        echo "Unsupported shell: $SHELL_NAME"
        echo "Supported shells: bash, zsh, fish"
        echo "Generate completions manually with: windwarden completions <shell>"
        exit 1
        ;;
esac
```

### Usage

```bash
# Make executable
chmod +x install-windwarden-completions.sh

# Run installer
./install-windwarden-completions.sh
```

## Integration with Package Managers

### Homebrew Formula

If WindWarden is distributed via Homebrew, completions can be installed automatically:

```ruby
class Windwarden < Formula
  # ... other formula content ...
  
  def install
    bin.install "windwarden"
    
    # Generate and install completions
    output = Utils.safe_popen_read("#{bin}/windwarden", "completions", "bash")
    (bash_completion/"windwarden").write output
    
    output = Utils.safe_popen_read("#{bin}/windwarden", "completions", "zsh")
    (zsh_completion/"_windwarden").write output
    
    output = Utils.safe_popen_read("#{bin}/windwarden", "completions", "fish")
    (fish_completion/"windwarden.fish").write output
  end
end
```

### NPM Package

If distributed via npm, include a postinstall script:

```json
{
  "scripts": {
    "postinstall": "node scripts/install-completions.js"
  }
}
```

This comprehensive guide covers all aspects of installing and using WindWarden's shell completions across different platforms and shells!