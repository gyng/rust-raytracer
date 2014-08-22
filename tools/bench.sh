#!/bin/sh

for conf in cow bunny box teapot; do
    CONF_FILE="tools/conf/${conf}.json"
    test -e "$CONF_FILE" && {
        echo "=== $1 $conf ==="
        time "$1" "$CONF_FILE"
    }
done

