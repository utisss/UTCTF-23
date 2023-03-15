#!/usr/bin/env python3

import base64
import hashlib
import io
import subprocess
import sys
import shlex
import zipfile

def get_file(name, archive):
    return [file for file in archive.infolist() if file.filename == name][0]

zip_bytes = base64.b64decode(input())
zip_memory_file = io.BytesIO(zip_bytes)

archive = zipfile.ZipFile(zip_memory_file)
file = get_file("commands/command.txt", archive)
data = archive.read(file)
md5 = hashlib.md5(data).hexdigest()

if md5 == "0e491b13e7ca6060189fd65938b0b5bc":
    command_file = archive.open("commands/command.txt")
    command = shlex.split(command_file.read().decode())
    command_file.close()
    out = subprocess.run(command, capture_output=True)
    print(out.stdout.decode("UTF-8") + out.stderr.decode("UTF-8"))
else:
    print("Invalid Command")
