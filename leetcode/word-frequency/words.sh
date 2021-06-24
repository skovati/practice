#!/bin/sh
# Read from the file words.txt and output the word frequency list to stdout.
xargs -n1 < words.txt | sort | uniq -c | sort -r | awk '{print $2 " " $1}'
