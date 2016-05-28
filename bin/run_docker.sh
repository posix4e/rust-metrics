#!/bin/sh
TAG='posi/rust-metrics'
docker run -ti -v `pwd`:/rust-metrics-live \
$TAG

