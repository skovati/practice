#!/bin/sh
# Read from the file file.txt and output the tenth line to stdout.
[ $(cat file.txt | wc -l) -lt 10 ] || echo $(head -n10 file.txt | tail -n1)
