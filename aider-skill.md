# Aider Subagent Skill

This skill allows spawning Aider as a subagent for coding tasks.

## Installation

Aider needs to be installed first. Try:
```bash
pipx install aider-chat
```

If pipx fails, install in a virtual environment:
```bash
python3 -m venv ~/.aider-venv
~/.aider-venv/bin/pip install aider-chat
```

## Usage

To spawn an Aider session:

```bash
# Method 1: Using pipx-installed aider
aider

# Method 2: Using virtual environment
~/.aider-venv/bin/aider

# Method 3: As an OpenClaw subagent
openclaw exec -- bash -c "aider"
```

## As a Subagent

You can spawn Aider as a subagent using:

```bash
sessions_spawn --runtime subagent --task "Run aider for coding assistance" --command "aider"
```

Or from within OpenClaw assistant:
```javascript
sessions_spawn({
  task: "Help with coding using Aider",
  runtime: "subagent",
  command: "aider"
})
```

## Configuration

Add to your OpenClaw config if you want Aider as a default coding agent:
```json
{
  "codingAgents": {
    "aider": {
      "command": "aider",
      "enabled": true
    }
  }
}
```