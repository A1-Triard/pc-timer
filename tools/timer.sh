#!/bin/sh

set -eu

nasm -o timer.com timer.s
objdump -b binary -m i386 -M i8086 -z -D timer.com > timer
