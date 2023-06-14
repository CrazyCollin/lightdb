# lightkv

## Introduction
Lightkv is a distributed key-value storage system written in Rust. It is designed to be a high performance, reliable and easy to use key-value storage system, compatible with Time Series Data. Currently, it is still under development.

## Architecture
Separated into serverl parts as follows:
- Storage Engine manages the data storage and index
- Network Layer with Tokio process high concurrency network requests
- Raft Algorithm Crate for cluster support
- Protocol Crate for multiple protocols support
- (Optional) Time Series Data Storage Crate for time series data support

## Roadmap(WIP)

- High performance in network with Tokio async runtime

- RESP protocol support

- Redis compatible commands

- Cluster support (With Raft consensus algorithm)

- External crate for Time Series Data Storage

- Multiple memory index support

- Pure LSM tree storage engine support

- Integration with Apache OpenDAL, with cloud stoarge access support

## References

- [Redis](https://redis.io/)
- [Raft](https://raft.github.io/)
- [Tokio](https://tokio.rs/)
- [ToyDB](https://github.com/erikgrinaker/toydb)