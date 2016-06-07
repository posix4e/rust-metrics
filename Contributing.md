Contributing to rust-metrics
-----------------------

Please use `cargo fmt` as part of the commit process.

0. Check our issues for things marked ready to do
1. resemble the surrounding code, and
2. contain lines no longer than 120 columns.

Using Docker for development
----------------------------

You can use the project's [Dockerfile](Dockerfile) to get a working development environment.

##### Usage

Build the image locally:

```sh
./bin/build_docker
```
Everything is currently tagged with posi/rust-metrics

If you don't have rust setup and you want to run the examples run:

```sh
 ./bin/run_docker 
```

each of the examples:
- ws # Runs a webserver with prometheus metrics
is given a command in the PATH of the running docker container.
