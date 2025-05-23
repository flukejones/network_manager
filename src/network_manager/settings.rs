//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.Settings`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.Settings.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::{Connection, Result, proxy};

impl SettingsProxy<'_> {
    pub async fn new_from_path(
        device_path: zbus::zvariant::OwnedObjectPath,
        connection: &Connection,
    ) -> Result<SettingsProxy<'_>> {
        SettingsProxy::builder(connection)
            .path(device_path)?
            .build()
            .await
    }
}

#[proxy(
    default_path = "/org/freedesktop/NetworkManager/Settings",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Settings",
    assume_defaults = true
)]
pub trait Settings {
    /// AddConnection method
    fn add_connection(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// AddConnection2 method
    fn add_connection2(
        &self,
        settings: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
        flags: u32,
        args: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<(
        zbus::zvariant::OwnedObjectPath,
        std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
    )>;

    /// AddConnectionUnsaved method
    fn add_connection_unsaved(
        &self,
        connection: std::collections::HashMap<
            &str,
            std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        >,
    ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// GetConnectionByUuid method
    fn get_connection_by_uuid(&self, uuid: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// ListConnections method
    fn list_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// LoadConnections method
    fn load_connections(&self, filenames: &[&str]) -> zbus::Result<(bool, Vec<String>)>;

    /// ReloadConnections method
    fn reload_connections(&self) -> zbus::Result<bool>;

    /// SaveHostname method
    fn save_hostname(&self, hostname: &str) -> zbus::Result<()>;

    /// ConnectionRemoved signal
    #[zbus(signal)]
    fn connection_removed(&self, connection: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// NewConnection signal
    #[zbus(signal)]
    fn new_connection(&self, connection: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// CanModify property
    #[zbus(property)]
    fn can_modify(&self) -> zbus::Result<bool>;

    /// Connections property
    #[zbus(property)]
    fn connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// Hostname property
    #[zbus(property)]
    fn hostname(&self) -> zbus::Result<String>;

    /// VersionId property
    #[zbus(property)]
    fn version_id(&self) -> zbus::Result<u64>;
}
