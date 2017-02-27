# fcm
[![Travis](https://img.shields.io/travis/panicbit/fcm-rust.svg?style=flat-square)][travis]
[![Coveralls](https://img.shields.io/coveralls/panicbit/fcm-rust.svg?style=flat-square)][coveralls]
[![Crates.io Version](https://img.shields.io/crates/v/fcm.svg?style=flat-square)][crates.io]
[![Crates.io Downloads](https://img.shields.io/crates/dv/fcm.svg?style=flat-square)][crates.io]
[![Crates.io License](https://img.shields.io/crates/l/fcm.svg?style=flat-square)][crates.io]

[crates.io]: https://crates.io/crates/fcm
[travis]: https://travis-ci.org/panicbit/fcm-rust
[coveralls]: https://coveralls.io/github/panicbit/fcm-rust


## Usage

Add this to `Cargo.toml`:

```rust
[dependencies]
fcm = "0.1.0"
tokio-core = "0.1"
futures = "0.1"
```

then add this to your crate root:

```rust
extern crate fcm;
extern crate tokio;
extern crate futures;
```

## Examples:

Here is an example to send out a FCM Message with some custom data:

```rust
use std::collections::HashMap;
use tokio_core::reactor::Core;
use fcm::{Client, MessageBuilder};

let mut core = Core::new().unwrap();
let handle = core.handle();
let client = fcm::Client::new(&handle);

let mut map = HashMap::new();
map.insert("message", "Howdy!");

let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
builder.data(map);

let work = client.send(builder.finalize());

match core.run(work) {
    Ok(response) => println!("Sent: {:?}", response),
    Err(error) => println!("Error: {:?}", error),
};
```

To send a message using FCM Notifications, we first build the notification:

```rust
use std::collections::HashMap;
use tokio_core::reactor::Core;
use fcm::{Client, NotificationBuilder};

let mut core = Core::new().unwrap();
let handle = core.handle();
let client = Client::new(&handle);

let mut builder = NotificationBuilder::new("Hey!");
builder.body("Do you want to catch up later?");
let notification = builder.finalize();
```

And then set it in the message, before sending it:

```rust
let mut builder = MessageBuilder::new("<FCM API Key>", "<registration id>");
builder.notification(notification);

let work = client.send(builder.finalize());
let result = core.run(work);
```
