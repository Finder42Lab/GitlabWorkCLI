#[cfg(target_os = "linux")]
use notify_rust;

#[cfg(target_os = "macos")]
use mac_notification_sys as mac_notify;

use crate::LogError;
use std::thread::spawn;

pub struct Notifier {}

impl Notifier {
    pub fn notify(
        summary: String,
        body: Option<String>,
        action: Vec<(String, String)>,
        on_action: fn(action: String),
    ) {
        #[cfg(target_os = "linux")]
        Self::notify_linux(summary, body, action, on_action);

        #[cfg(target_os = "macos")]
        Self::notify_macos(summary, body, action, on_action);
    }

    #[cfg(target_os = "linux")]
    fn notify_linux(
        summary: String,
        body: Option<String>,
        action: Vec<(String, String)>,
        on_action: fn(action: String),
    ) {
        let mut notifier = notify_rust::Notification::new();
        notifier.summary(&summary);

        if let Some(body) = body {
            notifier.body(&body);
        }

        for action in &action {
            notifier.action(&action.0, &action.1);
        }

        let handler = notifier.show().log_error();

        if !action.is_empty() && handler.is_ok() {
            spawn(move || {
                handler
                    .unwrap()
                    .wait_for_action(|action| on_action(action.to_string()))
            });
        }
    }

    #[cfg(target_os = "macos")]
    fn notify_macos(
        summary: String,
        body: Option<String>,
        action: Vec<(String, String)>,
        on_action: fn(action: String),
    ) {
        let bundle = mac_notify::get_bundle_identifier_or_default("gw-backend");
        mac_notify::set_application(&bundle)
            .log_error()
            .unwrap();

        let mut notification = mac_notify::Notification::new();

        if !action.is_empty() {
            notification.main_button(mac_notify::MainButton::DropdownActions(
                "Действия",
                action.iter().map(|s| s.1).collect(),
            ));
        }
        spawn(move || {
            let resp = mac_notify::send_notification(
                &summary,
                None,
                &body.unwrap_or("".to_string()),
                Some(&notification),
            );

            match resp {
                Ok(mac_notify::NotificationResponse::ActionButton(
                    action_title,
                )) => {
                    let action_code = action
                        .iter()
                        .find(|s| s.1 == action_title);

                    match action_code {
                        Some(action_code) => {
                            on_action(action_code.0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    }
}
