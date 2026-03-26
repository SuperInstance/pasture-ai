#!/usr/bin/env python3
import openai
import os

# Read API key from config
import yaml

with open(os.path.expanduser("~/.aider.conf.yml")) as f:
    config = yaml.safe_load(f)

api_key = config.get("openai-api-key")
api_base = config.get("openai-api-base", "https://api.deepseek.com")
model = config.get("model", "deepseek-chat")

print(f"API Key: {api_key[:10]}...")
print(f"API Base: {api_base}")
print(f"Model: {model}")

# Test the API
client = openai.OpenAI(
    api_key=api_key,
    base_url=api_base
)

try:
    response = client.chat.completions.create(
        model=model,
        messages=[{"role": "user", "content": "Say hello!"}],
        max_tokens=10
    )
    print(f"✅ Success! Response: {response.choices[0].message.content}")
except Exception as e:
    print(f"❌ Error: {e}")