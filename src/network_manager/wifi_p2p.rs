//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.Device.WifiP2P`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.WifiP2P.xml`.
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

impl WifiP2PProxy<'_> {
    pub async fn new_from_path(
        device_path: zbus::zvariant::OwnedObjectPath,
        connection: &Connection,
    ) -> Result<WifiP2PProxy<'_>> {
        WifiP2PProxy::builder(connection)
            .path(device_path)?
            .build()
            .await
    }
}

#[proxy(
    default_path = "/org/freedesktop/NetworkManager/Device/WifiP2P",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.WifiP2P",
    assume_defaults = true
)]
pub trait WifiP2P {
    /// StartFind method
    fn start_find(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// StopFind method
    fn stop_find(&self) -> zbus::Result<()>;

    /// PeerAdded signal
    #[zbus(signal)]
    fn peer_added(&self, peer: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// PeerRemoved signal
    #[zbus(signal)]
    fn peer_removed(&self, peer: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// HwAddress property
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Peers property
    #[zbus(property)]
    fn peers(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}
