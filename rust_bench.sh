#!/bin/bash
for i in {1..1000}
do
    ./target/release/rule110 > /dev/null
done