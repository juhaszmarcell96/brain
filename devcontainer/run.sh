#!/bin/bash

docker run --rm -it \
    --name zcalc \
    --user=root \
    -p 4444:4444 \
    -v "$(pwd)/../:/root/brain" \
    --entrypoint /bin/bash \
    brain:active
