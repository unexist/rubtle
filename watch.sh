#!/bin/zsh
CARGOFLAGS="-c -d 4 -q"
TESTFLAGS="--test-threads 1 --nocapture"

if [ "$#" -ne 1 ] ; then
    cargo watch $(echo $CARGOFLAGS) -x "test -p rubtle-lib -- $TESTFLAGS"
else
    RUSTFLAGS="-Z sanitizer=address" cargo watch $(echo $CARGOFLAGS) -x "test -p rubtle-lib -- $TESTFLAGS"
    #RUST_BACKTRACE=1 RUSTFLAGS="-Z sanitizer=address" cargo watch $(echo $CARGOFLAGS) -x "test -p rubtle-lib -- $TESTFLAGS"
fi
