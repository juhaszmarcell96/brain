docker run --rm -it `
    --name brain `
    --user=root `
    -p 4444:4444 `
    -v $PWD/../:/root/brain `
    --entrypoint /bin/bash `
    brain:active
