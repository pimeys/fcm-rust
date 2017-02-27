#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};

/// This struct represents a FCM notification. Use the 
/// corresponding `NotificationBuilder` to get an instance. You can then use 
/// this notification instance when sending a FCM message.
#[derive(Debug, PartialEq, Clone)]
pub struct Notification {
    title: Option<String>,
    body: Option<String>,
    icon: Option<String>,
    sound: Option<String>,
    badge: Option<String>,
    tag: Option<String>,
    color: Option<String>,
    click_action: Option<String>,
    body_loc_key: Option<String>,
    body_loc_args: Option<Vec<String>>,
    title_loc_key: Option<String>,
    title_loc_args: Option<Vec<String>>,
}

impl ToJson for Notification {
    fn to_json(&self) -> Json {
        let mut root = BTreeMap::new();

        if self.title.is_some() {
            root.insert("title".to_string(), self.title.clone().unwrap().to_json());
        }

        if self.icon.is_some() {
            root.insert("icon".to_string(), self.icon.clone().unwrap().to_json());
        }

        if self.body.is_some() {
            root.insert("body".to_string(), self.body.clone().unwrap().to_json());
        }

        if self.sound.is_some() {
            root.insert("sound".to_string(), self.sound.clone().unwrap().to_json());
        }

        if self.badge.is_some() {
            root.insert("badge".to_string(), self.badge.clone().unwrap().to_json());
        }

        if self.tag.is_some() {
            root.insert("tag".to_string(), self.tag.clone().unwrap().to_json());
        }

        if self.color.is_some() {
            root.insert("color".to_string(), self.color.clone().unwrap().to_json());
        }

        if self.click_action.is_some() {
            root.insert("click_action".to_string(), self.click_action.clone().unwrap().to_json());
        }

        if self.body_loc_key.is_some() {
            root.insert("body_loc_key".to_string(), self.body_loc_key.clone().unwrap().to_json());
        }

        if self.body_loc_args.is_some() {
            let body_loc_args_str = self.body_loc_args.clone().unwrap().to_json().to_string();
            root.insert("body_loc_args".to_string(), Json::String(body_loc_args_str));
        }

        if self.title_loc_key.is_some() {
            root.insert("title_loc_key".to_string(), self.title_loc_key.clone().unwrap().to_json());
        }

        if self.title_loc_args.is_some() {
            let title_loc_args_str = self.title_loc_args.clone().unwrap().to_json().to_string();
            root.insert("title_loc_args".to_string(), Json::String(title_loc_args_str));
        }

        Json::Object(root)
    }
}

/// A builder to get a `Notification` instance.
///
/// # Examples
///
/// ```rust
/// use fcm::NotificationBuilder;
///
/// let mut builder = NotificationBuilder::new();
//  builder.title("Australia vs New Zealand");
/// builder.body("3 runs to win in 1 ball");
/// let notification = builder.finalize();
/// ```
pub struct NotificationBuilder {
    title: Option<String>,
    body: Option<String>,
    icon: Option<String>,
    sound: Option<String>,
    badge: Option<String>,
    tag: Option<String>,
    color: Option<String>,
    click_action: Option<String>,
    body_loc_key: Option<String>,
    body_loc_args: Option<Vec<String>>,
    title_loc_key: Option<String>,
    title_loc_args: Option<Vec<String>>,
}

impl NotificationBuilder {
    /// Get a new `NotificationBuilder` instance, with a title.
    pub fn new() -> NotificationBuilder {
        NotificationBuilder {
            title: None,
            body: None,
            icon: None,
            sound: None,
            badge: None,
            tag: None,
            color: None,
            click_action: None,
            body_loc_key: None,
            body_loc_args: None,
            title_loc_key: None,
            title_loc_args: None,
        }
    }

    // Set the title of the notification
    pub fn title<S: Into<String>>(&mut self, title: S) -> &mut NotificationBuilder {
        self.title = Some(title.into());
        self
    }

    /// Set the body of the notification
    pub fn body<S: Into<String>>(&mut self, body: S) -> &mut NotificationBuilder {
        self.body = Some(body.into());
        self
    }

    /// Set the notification icon.
    pub fn icon<S: Into<String>>(&mut self, icon: S) -> &mut NotificationBuilder {
        self.icon = Some(icon.into());
        self
    }

    /// Set the sound to be played
    pub fn sound<S: Into<String>>(&mut self, sound: S) -> &mut NotificationBuilder {
        self.sound = Some(sound.into());
        self
    }

    /// Set the badge for iOS notifications
    pub fn badge<S: Into<String>>(&mut self, badge: S) -> &mut NotificationBuilder {
        self.badge = Some(badge.into());
        self
    }

    /// Tagging a notification allows you to replace existing notifications
    /// with the same tag with this new notification
    pub fn tag<S: Into<String>>(&mut self, tag: S) -> &mut NotificationBuilder {
        self.tag = Some(tag.into());
        self
    }

    /// The color of the icon, in #rrggbb format
    pub fn color<S: Into<String>>(&mut self, color: S) -> &mut NotificationBuilder {
        self.color = Some(color.into());
        self
    }

    /// What happens when the user clicks on the notification. Refer to 
    /// https://developers.google.com/cloud-messaging/http-server-ref#table2 for
    /// details.
    pub fn click_action<S: Into<String>>(&mut self, click_action: S) -> &mut NotificationBuilder {
        self.click_action = Some(click_action.into());
        self
    }

    /// Set the body key string for localization
    pub fn body_loc_key<S: Into<String>>(&mut self, body_loc_key: S) -> &mut NotificationBuilder {
        self.body_loc_key = Some(body_loc_key.into());
        self
    }

    /// String value to replace format specifiers in the body string.
    pub fn body_loc_args<S: Into<String>>(&mut self, body_loc_args: Vec<S>) -> &mut NotificationBuilder {
        self.body_loc_args = Some(body_loc_args.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Set the title key string for localization
    pub fn title_loc_key<S: Into<String>>(&mut self, title_loc_key: S) -> &mut NotificationBuilder {
        self.title_loc_key = Some(title_loc_key.into());
        self
    }

    /// String value to replace format specifiers in the title string.
    pub fn title_loc_args<S: Into<String>>(&mut self, title_loc_args: Vec<S>) -> &mut NotificationBuilder {
        self.title_loc_args = Some(title_loc_args.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Complete the build and get a `Notification` instance
    pub fn finalize(self) -> Notification {
        Notification {
            title: self.title,
            body: self.body,
            icon: self.icon,
            sound: self.sound,
            badge: self.badge,
            tag: self.tag,
            color: self.color,
            click_action: self.click_action,
            body_loc_key: self.body_loc_key,
            body_loc_args: self.body_loc_args.clone(),
            title_loc_key: self.title_loc_key,
            title_loc_args: self.title_loc_args.clone(),
        }
    }
}
