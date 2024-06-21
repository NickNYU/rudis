
# Rust Redis Server

A Redis server implemented in Rust. This project aims to provide a high-performance, reliable, and fully-featured Redis server, taking advantage of Rust's safety and concurrency features.

## Features

- Full Redis protocol implementation
- High-performance and low-latency
- Asynchronous I/O with Tokio
- Support for all standard Redis data types (strings, lists, sets, hashes, etc.)
- Pub/Sub messaging
- Persistence using RDB and AOF
- Comprehensive test suite

## Installation

To build and run the Redis server, you need to have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).

1. Clone the repository:
```sh
git clone https://github.com/NickNYU/rudis.git
cd rudis
```

2. Build the project:
```sh
cargo build --release
```

3. Run the server:
```sh
./target/release/rust-redis-server
```

## Usage

Once the server is running, you can interact with it using any Redis client. Here is an example using the Redis CLI:

```sh
redis-cli -p 6379
```

You can now execute Redis commands as usual:

```sh
SET mykey "Hello, Redis!"
GET mykey
```

## Configuration

The server can be configured using a configuration file. By default, the server looks for a `redis.conf` file in the current directory. You can specify a different configuration file using the `--config` option:

```sh
./target/release/rust-redis-server --config /path/to/your/redis.conf
```

## Development

### Running Tests

To run the test suite, use the following command:

```sh
cargo test
```

### Code Formatting

Ensure your code is properly formatted before committing:

```sh
cargo fmt
```

### Linting

Check for common mistakes and enforce coding standards:

```sh
cargo clippy
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any changes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- Thanks to the original Redis authors for their work on Redis.
- Inspired by the Rust community and existing projects.

## Contact

For any questions or feedback, please reach out to [cz739@nyu.edu].

## License

```plaintext
MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
