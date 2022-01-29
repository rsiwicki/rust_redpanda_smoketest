# Smoketest Rust, Rdkafka and Redpanda

Basic smoketest to proove that Rust using the rdkafka library can talk successfully to Redpanda.

Clone the repo

```
git clone https://github.com/rsiwicki/rust_redpanda_smoketest.git
```

Run the build

```
cd rust_redpanda_smoketest
cargo run
```

Create a test topic in Redpanda - e.g.:

```
rpk topic create cute-pandas --replicas 1
```

Point the demo at your Redpanda instances by modifying KAFKA_BROKER_PORT const in main.rs (later I might use the rust clap lib for a command line interface, though for now it's hardwired).

```
./target/debug/rust_redpanda_demo
```

You should see output indicating that the test is both producing and consuming. If not errors.


