#!/usr/bin/env python3
"""
Aider Bridge for OpenClaw
Allows OpenClaw to communicate with Dockerized Aider
"""

import subprocess
import os
import tempfile
import sys
import json
import shlex

def run_aider_docker(message, files=None, workdir=None):
    """
    Run Aider in Docker with a specific message
    Returns: (success, output)
    """
    if workdir is None:
        workdir = os.getcwd()
    
    if files is None:
        files = []
    
    # Build Docker command
    cmd = [
        "docker", "run", "--rm",
        "-v", f"{workdir}:/home/aider/code",
        "-v", f"{os.path.expanduser('~/.aider.conf.yml')}:/home/aider/.aider.conf.yml",
        "-w", "/home/aider/code",
        "aider-sandbox",
        "aider"
    ]
    
    # Add files
    cmd.extend(files)
    
    # Add message if provided
    if message:
        cmd.extend(["--message", message])
    
    # Add auto-accept flags
    cmd.extend(["--yes-always"])
    
    try:
        print(f"Running: {' '.join(shlex.quote(arg) for arg in cmd)}", file=sys.stderr)
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=300,  # 5 minute timeout
            cwd=workdir
        )
        
        return result.returncode == 0, result.stdout + result.stderr
    except subprocess.TimeoutExpired:
        return False, "Timeout: Aider took too long to respond"
    except Exception as e:
        return False, f"Error: {str(e)}"

def main():
    """Command-line interface"""
    if len(sys.argv) < 2:
        print("Usage: aider-bridge.py 'message' [file1 file2 ...]")
        sys.exit(1)
    
    message = sys.argv[1]
    files = sys.argv[2:] if len(sys.argv) > 2 else None
    
    success, output = run_aider_docker(message, files)
    
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