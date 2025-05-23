//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.PPP`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.PPP.xml`.
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

impl PPPProxy<'_> {
    pub async fn new_from_path(
        device_path: zbus::zvariant::OwnedObjectPath,
        connection: &Connection,
    ) -> Result<PPPProxy<'_>> {
        PPPProxy::builder(connection)
            .path(device_path)?
            .build()
            .await
    }
}

#[proxy(
    default_path = "/org/freedesktop/NetworkManager/PPP",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.PPP",
    assume_defaults = true
)]
pub trait PPP {
    /// NeedSecrets method
    fn need_secrets(&self) -> zbus::Result<(String, String)>;

    /// SetIfindex method
    fn set_ifindex(&self, ifindex: i32) -> zbus::Result<()>;

    /// SetIp4Config method
    fn set_ip4_config(
        &self,
        config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetIp6Config method
    fn set_ip6_config(
        &self,
        config: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// SetState method
    fn set_state(&self, state: u32) -> zbus::Result<()>;
}
