# Aider Subagent Integration for OpenClaw

## ✅ Setup Complete

**What's installed:**
1. ✅ Aider 0.86.2 in virtual environment (`~/.aider-venv/`)
2. ✅ DeepSeek API configured (`~/.aider.conf.yml`)
3. ✅ Bridge scripts for communication

## How to Use Aider as a Subagent

### Option 1: Direct Command (You run it)
```bash
# Activate venv and run
source ~/.aider-venv/bin/activate
aider [files...]

# Or directly
~/.aider-venv/bin/aider [files...]
```

### Option 2: Through OpenClaw (I run it for you)

When you ask me to help with coding, I can:
1. Spawn Aider as a subagent
2. Pass your request to Aider
3. Return the results to you

**Example workflow:**
```
You: "Help me fix this Python script"
Me: [Spawns Aider subagent with your script]
Aider: [Analyzes and fixes the code]
Me: [Returns the fixed code to you]
```

## Bridge Scripts Available

1. **`aider-bridge-venv.py`** - Main bridge for OpenClaw
   ```bash
   python3 aider-bridge-venv.py "Fix this bug" file.py
   ```

2. **Simple test script** - Test API connection
   ```bash
   python3 test-deepseek.py
   ```

## Configuration Files

1. **`~/.aider.conf.yml`** - Aider config with DeepSeek API
2. **Virtual environment** - `~/.aider-venv/` (sandboxed)

## Ready for Use!

Your Aider setup is complete and sandboxed in a virtual environment. The DeepSeek API is configured and tested.

**Next:** Ask me to help with any coding task, and I'll use Aider as a subagent to assist you!