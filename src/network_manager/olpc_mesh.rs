//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.Device.OlpcMesh`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.OlpcMesh.xml`.
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

impl OlpcMeshProxy<'_> {
    pub async fn getnew_from_path_proxy(
        device_path: zbus::zvariant::OwnedObjectPath,
        connection: &Connection,
    ) -> Result<OlpcMeshProxy<'_>> {
        OlpcMeshProxy::builder(connection)
            .path(device_path)?
            .build()
            .await
    }
}

#[proxy(
    default_path = "/org/freedesktop/NetworkManager/Device/OlpcMesh",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.OlpcMesh",
    assume_defaults = true
)]
pub trait OlpcMesh {
    /// ActiveChannel property
    #[zbus(property)]
    fn active_channel(&self) -> zbus::Result<u32>;

    /// Companion property
    #[zbus(property)]
    fn companion(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// HwAddress property
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;
}
