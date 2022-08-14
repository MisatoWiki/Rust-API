#!/bin/sh
source $PWD/.env && env | grep MISATO_TOKEN
source $PWD/.env && cargo test
