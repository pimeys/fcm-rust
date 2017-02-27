extern crate rustc_serialize;

use rustc_serialize::json::{ToJson};
use {NotificationBuilder};

#[test]
fn should_set_notification_title() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.title, None);

    let mut builder = NotificationBuilder::new();
    builder.title("title");
    let nm = builder.finalize();

    assert_eq!(nm.title, Some("title".to_string()));
}
#[test]
fn should_set_notification_body() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.body, None);

    let mut builder = NotificationBuilder::new();
    builder.body("body");
    let nm = builder.finalize();

    assert_eq!(nm.body, Some("body".to_string()));
}

#[test]
fn should_set_notification_icon() {
    let mut builder = NotificationBuilder::new();
    builder.icon("newicon");
    let nm = builder.finalize();

    assert_eq!(nm.icon, Some("newicon".to_string()));
}

#[test]
fn should_set_notification_sound() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.sound, None);

    let mut builder = NotificationBuilder::new();
    builder.sound("sound.wav");
    let nm = builder.finalize();

    assert_eq!(nm.sound, Some("sound.wav".to_string()));
}

#[test]
fn should_set_notification_badge() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.badge, None);

    let mut builder = NotificationBuilder::new();
    builder.badge("1");
    let nm = builder.finalize();

    assert_eq!(nm.badge, Some("1".to_string()));
}

#[test]
fn should_set_notification_tag() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.tag, None);

    let mut builder = NotificationBuilder::new();
    builder.tag("tag");
    let nm = builder.finalize();

    assert_eq!(nm.tag, Some("tag".to_string()));
}

#[test]
fn should_set_notification_color() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.color, None);

    let mut builder = NotificationBuilder::new();
    builder.color("color");
    let nm = builder.finalize();

    assert_eq!(nm.color, Some("color".to_string()));
}

#[test]
fn should_set_notification_click_action() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.click_action, None);

    let mut builder = NotificationBuilder::new();
    builder.click_action("action");
    let nm = builder.finalize();

    assert_eq!(nm.click_action, Some("action".to_string()));
}

#[test]
fn should_set_notification_body_loc_key() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.body_loc_key, None);

    let mut builder = NotificationBuilder::new();
    builder.body_loc_key("key");
    let nm = builder.finalize();

    assert_eq!(nm.body_loc_key, Some("key".to_string()));
}

#[test]
fn should_set_notification_body_loc_args() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.body_loc_args, None);

    let mut builder = NotificationBuilder::new();
    builder.body_loc_args(vec!["args"]);
    let nm = builder.finalize();

    assert_eq!(nm.body_loc_args, Some(vec!["args".to_string()]));
    assert_eq!(nm.to_json().search("body_loc_args").unwrap().as_string(), Some("[\"args\"]"));
}

#[test]
fn should_set_notification_title_loc_key() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.title_loc_key, None);

    let mut builder = NotificationBuilder::new();
    builder.title_loc_key("key");
    let nm = builder.finalize();

    assert_eq!(nm.title_loc_key, Some("key".to_string()));
}

#[test]
fn should_set_notification_title_loc_args() {
    let nm = NotificationBuilder::new().finalize();

    assert_eq!(nm.title_loc_args, None);

    let mut builder = NotificationBuilder::new();
    builder.title_loc_args(vec!["args"]);
    let nm = builder.finalize();

    assert_eq!(nm.title_loc_args, Some(vec!["args".to_string()]));
    assert_eq!(nm.to_json().search("title_loc_args").unwrap().as_string(), Some("[\"args\"]"));
}
