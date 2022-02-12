#!/usr/bin/env python3

import os
import subprocess
import time
import http.server
import socketserver


def file_times(path):
  for root, _, files in os.walk(path):
    for file in files:
      yield os.stat(os.path.join(root, file)).st_mtime


def build():
  try:
    os.system("rm -rf web")
    os.system("make web")
    os.system("mkdir web")
    os.system("cp src/index.html web")
    os.system("cp target/wasm32-unknown-emscripten/release/deps/sdl_emscripten_template.wasm web/")
    os.system("cp target/wasm32-unknown-emscripten/release/deps/sdl_emscripten_template.js web/")
  except:
    quit()


def run():
  return subprocess.Popen("python3 -m http.server --bind 0.0.0.0 --directory web", shell=True)


def get_output(func, proc):
  if proc.stdout is not None:
    for line in proc.stdout.splitlines():
      print(f"[{func}] {line}")


if __name__ == '__main__':
  build()
  process = run()
  last_modified = max(file_times("./src"))

  while True:
    current_modified = max(file_times("./src"))
    if current_modified > last_modified:
      last_modified = current_modified
      print("\nRestarting process.")
      process.kill()
      build()
      process = run()

    time.sleep(1)
