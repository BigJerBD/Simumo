# Simumo - simulator

The simulator generate all the metrics.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

To run the simulator you must install

```
Rustup
```
Also, you must have a config file in .json or .yaml.

See ./etc/config.yaml as example.

### Installing

There is only one environnement at the moment.

First of all, you need to setup your virtual environnement because we use our own python package.

if you are on __Linux__
run
```
make-dev
```
This will setup your virtual environnement.

if you are on __Window__

You will have to execute manually the makefile.

To do so execute in the terminal

1. virtualenv venv --no-site-packages
2. pip install -r simumap/requirements.txt
3. pip install ./simumap
4. source venv/bin/activate

À partir d'ici, vous vous retrouvez dans l'envionnement virtual.

To run the simulator execute in the terminal

```
cargo run -- -c config.yaml
```

Eventually, the simulator will output the metrics as data.

## Generating auto documentation

Execute in the terminal

```
cargo doc --open --no-deps --document-private-items
```

## Running the tests
Execute this command in terminal

```
cargo test
```

## Code format

We use cargoFmt.

### Break down into end to end tests

At the moment, the simulator only has unit test.

### And coding style tests

All unit test are create within a module. 

```
See lane_graph.rs tests as example.
```

## Deployment

At the moment, simumo doesn't support deployment.

## Built With

* [Cargo] - Rust's package manager.

## Versioning

At the moment versioning isn't support.

## Authors

See CODEOWNERS file at root level.

## Divers

