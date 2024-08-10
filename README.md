VITA
----

A workflow scheduler for automating crucial or every-day tasks

---

## Table of Contents

- [Introduction](#introduction)
- [Crates](#prerequisites)
- [Installation](#installation)
    - [Build & Run](#build-and-run-binaries)
    - [Run with Cargo](#alternatively-you-can-run-directly-with-cargo)

## Introduction

**⚠️ Warning:** The project is actively being worked on and is very much still incomplete. Some things may not work as
expected or not at all.

---

Vita allows you to split your tasks in scripts and
execute them in varying schedules (like cron). The project is completely modular and all its components are split into
their own crates [see crates](#crates).

## Crates

The project is made up of the following crates. The crates are as minimal as possible and have little to no
dependencies:

### [vita-scheduler](./crates/scheduler)

> Inspired by [clockwerk](https://github.com/onatm/clockwerk)

Allows for intuitive syntax/schedule creation like the following:

```rust
let scheduler = schedule_every! {
    1.week(), 1.day() => {
        println!("Every week and every day!!")
    },
    
    1.day() + 4.hour() => {
        println!("Every 28 hours!")
    }

    Monday.midnight() => {
        println!("Every Monday at midnight!")
    },
}
```

### [vita-socket](./crates/socket)

An abstraction on top of [Unix Sockets (UDS)](https://en.wikipedia.org/wiki/Unix_domain_socket), allowing for
inter-communication between the server and the clients

### [vita-daemon](./crates/daemon)

vita-daemon employs [New-Style Daemons](https://www.freedesktop.org/software/systemd/man/latest/daemon.html) for the
server, as the scheduler and executor should always be active as a background process

### [vita-parser]()

> TODO ⚙️

## Installation

To begin, clone the repository and cd into its directory:

```console
$ git clone https://github.com/douris/vita
...

$ cd vita/ 
```

### Build and run binaries

The program is split in two binaries for now:

- Server
    ```console
    $ cargo build --release --manifest-path=./Cargo.toml --bin=vita-server
    ```
- Client
    ```console
    $ cargo build --release --manifest-path=./Cargo.toml --bin=vita-client
    ```

In the end, 2 binaries will be created **vita-server** and **vita-client** and both can be found at `./target/release/`

To run the binaries directly, cd into the above directory and run:

```console
$ ./vita-server 

# or

$ ./vita-client
```

### Alternatively, you can run directly with Cargo

- Server
    ```console
    $ cargo run --release --manifest-path=./Cargo.toml --bin=vita-server
    ```
- Client
    ```console
    $ cargo run --release --manifest-path=./Cargo.toml --bin=vita-client
    ```
