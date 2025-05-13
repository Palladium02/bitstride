# Bitstride (WIP)

Bitstride is a load balancer written in Rust that employs a custom metric to efficiently distribute the load onto all available nodes. Bitstride consists of three components. First the balancer itself, secondly a daemon that needs to run on each node to provide information about system resources so the balancer can make educated routing decisions and lastly a web ui that allows for monitoring of nodes and changing the configuration of the balancer.

## Building

Run `cargo build -p balancer --release` to build the balancer or `cargo build -p daemon --release` to build the daemon respectively.
