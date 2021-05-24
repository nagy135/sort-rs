#!/bin/bash

bspc rule -a "*" --one-shot state=floating
cargo run
