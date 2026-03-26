#!/usr/bin/env python3
"""
Aider Bridge for OpenClaw (Virtual Environment version)
Allows OpenClaw to communicate with sandboxed Aider
"""

import subprocess
import os
import sys
import shlex
import tempfile

def run_aider_venv(message, files=None, workdir=None):
    """
    Run Aider from virtual environment with a specific message
    Returns: (success, output)
    """
    if workdir is None:
        workdir = os.getcwd()
    
    if files is None:
        files = []
    
    # Path to virtual environment
    venv_path = os.path.expanduser("~/.aider-venv")
    aider_path = os.path.join(venv_path, "bin", "aider")
    
    # Build command
    cmd = [aider_path]
    
    # Add files
    cmd.extend(files)
    
    # Add message
    if message:
        cmd.extend(["--message", message])
    
    # Add auto-accept flags
    cmd.extend(["--yes-always"])
    
    # Set environment
    env = os.environ.copy()
    env["PATH"] = f"{venv_path}/bin:{env['PATH']}"
    
    # Copy config if it exists
    config_source = os.path.expanduser("~/.aider.conf.yml")
    if os.path.exists(config_source):
        # Use the same config
        pass  # Aider will read from ~/.aider.conf.yml
    
    try:
        print(f"Running: {' '.join(shlex.quote(arg) for arg in cmd)}", file=sys.stderr)
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=300,  # 5 minute timeout
            cwd=workdir,
            env=env
        )
        
        return result.returncode == 0, result.stdout + result.stderr
    except subprocess.TimeoutExpired:
        return False, "Timeout: Aider took too long to respond"
    except Exception as e:
        return False, f"Error: {str(e)}"

def main():
    """Command-line interface"""
    if len(sys.argv) < 2:
        print("Usage: aider-bridge-venv.py 'message' [file1 file2 ...]")
        print("Example: aider-bridge-venv.py 'Fix the bug in this code' app.py")
        sys.exit(1)
    
    message = sys.argv[1]
    files = sys.argv[2:] if len(sys.argv) > 2 else None
    
    # Check if virtual environment exists
    venv_path = os.path.expanduser("~/.aider-venv")
    if not os.path.exists(venv_path):
        print("❌ Virtual environment not found. Run setup-aider-venv.sh first.")
        sys.exit(1)
    
    success, output = run_aider_venv(message, files)
    
    if success:
        print("✅ Aider completed successfully")
        print("\n" + "="*80 + "\n")
        print(output)
    else:
        print("❌ Aider failed")
        print("\n" + "="*80 + "\n")
        print(output)
        sys.exit(1)

if __name__ == "__main__":
    main()