#!/bin/bash
for i in {1..1000}
do
    ./target/release/fnky > /dev/null
done