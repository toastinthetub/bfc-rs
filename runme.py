#!/usr/bin/env python3
import shutil
import sys

def check_cmd(cmd):
    return shutil.which(cmd) is not None

missing = []

for tool in ["nasm", "ld"]:
    if not check_cmd(tool):
        missing.append(tool)

if missing:
    print(f"missing required tools: {', '.join(missing)}")
    print("install them using your package manager (e.g., apt install nasm binutils)")
    sys.exit(1)
else:
    print("all required tools are installed")

