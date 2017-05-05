#!/bin/bash
set -x
while true;
do
    inotifywait -e modify,close_write,moved_to,create -r $1 |
        while read -r directory events filename;
        do
            cargo test -j1 --all
        done
done
