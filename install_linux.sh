#!/bin/sh
rm rgch;
wget https://github.com/Scstechr/rgch/raw/master/target/x86_64-unknown-linux-musl/release/rgch;
chmod u+x ./rgch
./rgch --version
