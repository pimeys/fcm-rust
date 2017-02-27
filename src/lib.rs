#![doc(html_root_url = "https://panicbit.github.io/fcm-rust/fcm/")]
//! fcm
//! ===
//!
//! # Usage:
//!
//! Add this to `Cargo.toml`:
//!
//! ```ignore
//! [dependencies]
//! fcm = "0.1.0"
//! ```
//!
//! then add this to your crate root:
//!
//! ```ignore
//! extern crate fcm;
//! extern crate futures;
//! extern crate tokio_core;
//! ```
//!
//! # Examples:
//!
//! Here is an example to send out a FCM Message with some custom data:
//!
//! ```no_run
//! # extern crate fcm;
//! # extern crate futures;
//! # extern crate tokio_core;
//!
//! # use std::collections::HashMap;
//! # fn main() {
//! let mut core = tokio_core::reactor::Core::new().unwrap();
//! let handle = core.handle();
//! let client = fcm::Client::new(&handle);
//!
//! let mut map = HashMap::new();
//! map.insert("message", "Howdy!");
//!
//! let mut builder = fcm::MessageBuilder::new("<FCM API Key>", "<registration id>");
//! builder.data(map);
//!
//! let work = client.send(builder.finalize());
//!
//! match core.run(work) {
//!     Ok(response) => println!("Sent: {:?}", response),
//!     Err(error) => println!("Error: {:?}", error),
//! };
//! # }
//! ```
//!
//! To send a message using FCM Notifications, we first build the notification:
//!
//! ```rust
//! # extern crate fcm;
//!
//! # fn main() {
//! let mut builder = fcm::NotificationBuilder::new();
//! builder.title("Hey!");
//! builder.body("Do you want to catch up later?");
//! let notification = builder.finalize();
//! # }
//! ```
//! And then set it in the message, before sending it:
//!
//! ```no_run
//! # extern crate fcm;
//! # extern crate futures;
//! # extern crate tokio_core;
//!
//! # fn main() {
//! let mut core = tokio_core::reactor::Core::new().unwrap();
//! let handle = core.handle();
//! let client = fcm::Client::new(&handle);
//!
//! let mut notification_builder = fcm::NotificationBuilder::new();
//! notification_builder.title("Hey!");
//! notification_builder.body("Do you want to catch up later?");
//!
//! let notification = notification_builder.finalize();
//! let mut message_builder = fcm::MessageBuilder::new("<FCM API Key>", "<registration id>");
//! message_builder.notification(notification);
//!
//! let work = client.send(message_builder.finalize());
//! let result = core.run(work);
//!
//! match result {
//!   Ok(response) => println!("message_id: {:?}", response.message_id),
//!   Err(error) => println!("Error: {:?}", error),
//! }
//! # }
//! ```

extern crate rustc_serialize;
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate hyper_tls;

mod message;
pub use message::*;
mod notification;
pub use notification::*;
mod client;
pub use client::*;

pub use client::response::FcmError as Error;
