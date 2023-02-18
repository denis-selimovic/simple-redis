# Redis clone

Simple Redis clone (both client & server) written in Rust for educational purposes.

## CLI

 Server can accept multiple connections spread out to the thread pool by using `async-std` library. Following commands can be used to start server and/or client. Server uses port `7878` to listen for incoming connections.

> `$ cargo run --release --bin server`

> `$ cargo run --release --bin client`

## Example usage

<p align="center" width="100%">
    <img width="70%" src="example/demo.gif"> 
</p>

## (De)Serialization

`RESP` (REdis Serialization Protocol) is implemented (almost) identically. For more info visit [RESP specification](https://redis.io/docs/reference/protocol-spec). Currently, following types are suppored:
- Simple strings (with `+` starting byte)
- Error type (with `-` starting byte)
- 64-bit signed integers (with `:` starting byte)
- Bulk strings (with `$` starting byte)
- Arrays (with `*` starting byte)
- NULL type (implemented as bulk string with length -1)
- All commands end with `!` as end-byte (this is where we diverge from `RESP` which has no end-of-instruction byte)

Other types can be added easily within current setup. You can write your own clients in any programming language by adhering to the protocol described above and use it with server written in Rust.


## Commands

Currently following commands are available:
- `GET <key>` to read in-memory storage
- `SET <key> <value>` to set given key to a value
- `DELETE <key>` to delete key from in-memory storage
- `FLUSH` to clear everything from in-memory storage
- `MGET <key1> <key2> <key3> ...` to get values from multiple keys
- `MSET <key1> <val1> <key2> <val2> <key3> <val3> ...` to set values for multiple keys


## Tests

To run tests use:

> `$ cargo test`
