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
```

then add this to your crate root:

```rust
extern crate fcm;
```

## Examples:

Here is an example to send out a FCM Message with some custom data:

```rust
use fcm::{Message, Client};
use std::collections::HashMap;

let client = Client::new();

let mut map = HashMap::new();
map.insert("message", "Howdy!");

let message = Message::new("<registration id>").data(map);
let result = client.send(message, "<FCM API Key>");
```

To send a message using FCM Notifications, we first build the notification:

```rust
use fcm::{Message, NotificationBuilder, Client};

let client = Client::new();

let notification = NotificationBuilder::new("Hey!")
    .body("Do you want to catch up later?")
    .finalize();
```

And then set it in the message, before sending it:

```rust
let message = Message::new("<registration id>")
    .notification(notification);

let result = client.send(message, "<FCM API Key>");
```

You can now handle the result accordingly:

```rust
match result {
  Ok(response) => println!("message_id: {:?}", response.message_id),
  Err(error) => println!("Error: {:?}", error),
}
```
