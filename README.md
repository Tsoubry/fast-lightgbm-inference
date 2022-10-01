# Fast Lightgbm Inference

This project contains a practical application of running a Lightgbm model in production for very quick inference.
Running the feature transformation and training the model with Lightgbm happens in Python. A deterministic data transformer is used for this, and the same one is reused in the application serving predictions. Both the feature pipeline config and the model can be updated without any downtime of the server.

There are 3 components:

1. feature-pipe: a library with the definitions of the feature config and deterministic data transformations written in Rust.
2. rust-transformer: containing Rust-Numpy bindings from the feature-pipe, along with a Python scikit-learn transformer and some other utility code. It compiles to the Python ODT module.
3. main application: Actix-Web application for making online predictions and updating model/configuration.


## Quickstart

1. Running the server

    Run 'cargo run' or build the image and run with Docker.

2. Training a model in Python and sending model/config to the server

    ```sh
    cargo install maturin
    cd rust-transformer
    maturin build
    cd ..
    poetry install
    ```
    And you should be good to go. 

    For an example: see the notebook in examples/breast-cancer.


## Running loadtests

locust -f main.py --host=http://localhost:8080


### With distributed workers

locust -f main.py --master --host=http://localhost:8080 --users=10 --spawn-rate=2

./locust_workers.sh
