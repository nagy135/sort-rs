#!/bin/bash

bspc rule -a "*" --one-shot state=floating
./target/debug/sort-rs
