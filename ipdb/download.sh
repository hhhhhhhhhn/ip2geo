#!/bin/sh
while read -r line; do wget "$line"; done <sources.txt
