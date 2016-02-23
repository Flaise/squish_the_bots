Squish the Bots
===============

To run the server:

    cargo run --bin squish_the_bots -- --web 8080 --simulation 9090

The -- in the middle separates the arguments meant for cargo from the arguments meant for squish_the_bots.

To have the example bot connect to the server:

    cargo run --bin hunter_bot -- 127.0.0.1:9090

You will need to run two bot instances or the round will not start.
