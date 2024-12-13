#!/usr/bin/env python3
import argparse
from os import makedirs
from os.path import exists, join
import subprocess
import shutil
from sys import stderr

APP_NAME = "xibao-gen"


parser = argparse.ArgumentParser(description="Installer script for xibao-gen. ")
parser.add_argument(
    "--prefix",
    default="/usr/local/",
    help="Directory to install into. "
)

args = parser.parse_args()
install_prefix = args.prefix
bin_prefix = join(install_prefix, "bin")
share_prefix = join(install_prefix, "share")

def installation():
    if not exists(install_prefix):
        makedirs(install_prefix, exist_ok=True)
    if not exists(bin_prefix):
        makedirs(bin_prefix, exist_ok=True)
    if not exists(share_prefix):
        makedirs(share_prefix, exist_ok=True)

    status = subprocess.run(["cargo", "build", "--release"])
    if status.returncode != 0:
        raise Exception("failed to build the project. ")

    shutil.copyfile(f"target/release/{APP_NAME}", join(bin_prefix, APP_NAME))
    shutil.copytree("resource", join(share_prefix, APP_NAME))

if __name__ == "__main__":
    try:
        installation()
    except BaseException as e:
        print("Fatal Error: ", e.args, file=stderr)

