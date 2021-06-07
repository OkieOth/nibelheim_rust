#!/bin/bash

scriptPos=${0%/*}

yacgVersion=1.3.0

if ! docker run --rm -t \
    -u $(id -u ${USER}):$(id -g ${USER}) \
    -v $(cd $scriptPos/.. && pwd)/src/definitions:/outputDir \
    -v $(cd $scriptPos/.. && pwd)/models:/modelDir \
    -v $(cd $scriptPos/.. && pwd)/yacg/templates:/templateDir \
    -v $(cd $scriptPos/.. && pwd)/yacg/config:/configDir \
    okieoth/yacg:$yacgVersion \
    --config /configDir/definitions.json; then
    echo "error while generate code"
    exit 1
fi
