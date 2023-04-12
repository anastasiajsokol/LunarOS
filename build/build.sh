#!/bin/sh

nasm -f bin bootloader/stage_one.s -o bin/stage_one.bin
nasm -f bin bootloader/stage_two.s -o bin/stage_two.bin
