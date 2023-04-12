#!/bin/sh

while getopts d:f: flag
do
    case "${flag}" in
        d) drive=${OPTARG};;
        f) file=${OPTARG};;
    esac
done

if [ -z "$file" ];
    then echo "No binary image specified with -f";
    else if [ -z "$drive" ];
            then qemu-system-x86_64 -drive format=raw,file=$file;
            else sudo dd if=$file of=$drive && sync;
        fi
fi
