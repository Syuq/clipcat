use clipcat_dbus_variant as dbus_variant;
use zbus::dbus_interface;

use crate::{notification, ClipboardWatcherToggle};

pub struct WatcherService<Notification> {
    watcher_toggle: ClipboardWatcherToggle<Notification>,
}

impl<Notification> WatcherService<Notification> {
    #[inline]
    pub const fn new(watcher_toggle: ClipboardWatcherToggle<Notification>) -> Self {
        Self { watcher_toggle }
    }
}

#[dbus_interface(name = "org.clipcat.clipcat.Watcher")]
impl<Notification> WatcherService<Notification>
where
    Notification: notification::Notification + 'static,
{
    fn enable(&self) -> dbus_variant::WatcherState {
        self.watcher_toggle.enable();
        self.watcher_toggle.state().into()
    }

    fn disable(&self) -> dbus_variant::WatcherState {
        self.watcher_toggle.disable();
        self.watcher_toggle.state().into()
    }

    fn toggle(&self) -> dbus_variant::WatcherState {
        self.watcher_toggle.toggle();
        self.watcher_toggle.state().into()
    }

    fn get_state(&self) -> dbus_variant::WatcherState { self.watcher_toggle.state().into() }
}
