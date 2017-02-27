use {MessageBuilder, Priority};
use notification::NotificationBuilder;
use std::collections::HashMap;

#[test]
fn should_create_new_message() {
    let msg = MessageBuilder::new("api_key", "token").finalize();

    assert_eq!(msg.to, "token");
}

#[test]
fn should_set_registration_ids() {
    let msg = MessageBuilder::new("api_key", "token").finalize();

    assert_eq!(msg.registration_ids, None);

    let mut builder = MessageBuilder::new("api_key", "token");
    builder.registration_ids(vec!["id1"]);
    let msg = builder.finalize();

    assert_eq!(msg.registration_ids, Some(vec!["id1".to_string()]));
}

#[test]
fn should_set_collapse_key() {
    let msg = MessageBuilder::new("api_key", "token").finalize();

    assert_eq!(msg.collapse_key, None);

    let mut builder = MessageBuilder::new("api_key", "token");
    builder.collapse_key("key");
    let msg = builder.finalize();

    assert_eq!(msg.collapse_key, Some("key".to_string()));
}

#[test]
fn should_set_priority() {
    let msg = MessageBuilder::new("api_key", "token").finalize();

    assert_eq!(msg.priority, None);

    let mut builder = MessageBuilder::new("api_key", "token");
    builder.priority(Priority::Normal);
    let msg = builder.finalize();

    assert_eq!(msg.priority, Some(Priority::Normal));
}

#[test]
fn should_set_content_available() {
    let msg = MessageBuilder::new("api_key", "token").finalize();

    assert_eq!(msg.content_available, None);

    let mut builder = MessageBuilder::new("api_key", "token");
    builder.content_available(true);
    let msg = builder.finalize();

    assert_eq!(msg.content_available, Some(true));
}

#[test]
fn should_set_delay_while_idle() {
    let msg = MessageBuilder::new("api_key", "token").finalize();

    assert_eq!(msg.delay_while_idle, None);

    let mut builder = MessageBuilder::new("api_key", "token");
    builder.delay_while_idle(true);
    let msg = builder.finalize();

    assert_eq!(msg.delay_while_idle, Some(true));
}

#[test]
fn should_set_time_to_live() {
    let msg = MessageBuilder::new("api_key", "token").finalize();

    assert_eq!(msg.time_to_live, None);

    let mut builder = MessageBuilder::new("api_key", "token");
    builder.time_to_live(10);
    let msg = builder.finalize();

    assert_eq!(msg.time_to_live, Some(10));
}

#[test]
fn should_set_restricted_package_name() {
    let msg = MessageBuilder::new("api_key", "token").finalize();

    assert_eq!(msg.restricted_package_name, None);

    let mut builder = MessageBuilder::new("api_key", "token");
    builder.restricted_package_name("name");
    let msg = builder.finalize();

    assert_eq!(msg.restricted_package_name, Some("name".to_string()));
}

#[test]
fn should_set_dry_run() {
    let msg = MessageBuilder::new("api_key", "token").finalize();

    assert_eq!(msg.dry_run, None);

    let mut builder = MessageBuilder::new("api_key", "token");
    builder.dry_run(true);
    let msg = builder.finalize();

    assert_eq!(msg.dry_run, Some(true));
}

#[test]
fn should_set_data() {
    let msg = MessageBuilder::new("api_key", "token").finalize();

    assert_eq!(msg.data, None);

    let mut data = HashMap::new();
    data.insert("my", "data");

    let mut builder = MessageBuilder::new("api_key", "token");
    builder.data(data);
    let msg = builder.finalize();

    assert_eq!(msg.data.unwrap().get("my"), Some(&"data".to_string()));
}

#[test]
fn should_set_notifications() {
    let msg = MessageBuilder::new("api_key", "token").finalize();

    assert_eq!(msg.notification, None);

    let nm = NotificationBuilder::new().finalize();

    let mut builder = MessageBuilder::new("api_key", "token");
    builder.notification(nm);
    let msg = builder.finalize();

    assert!(msg.notification != None);
}
