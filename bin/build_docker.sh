#!/bin/sh
set -ex
TAG='posi/rust-metrics'
docker build -t $TAG .
