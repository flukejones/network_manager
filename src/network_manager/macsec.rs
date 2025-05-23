//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.Device.Macsec`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.Macsec.xml`.
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

impl MacsecProxy<'_> {
    pub async fn new_from_path(
        device_path: zbus::zvariant::OwnedObjectPath,
        connection: &Connection,
    ) -> Result<MacsecProxy<'_>> {
        MacsecProxy::builder(connection)
            .path(device_path)?
            .build()
            .await
    }
}

#[proxy(
    default_path = "/org/freedesktop/NetworkManager/Device/Macsec",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Macsec",
    assume_defaults = true
)]
pub trait Macsec {
    /// CipherSuite property
    #[zbus(property)]
    fn cipher_suite(&self) -> zbus::Result<u64>;

    /// EncodingSa property
    #[zbus(property)]
    fn encoding_sa(&self) -> zbus::Result<u8>;

    /// Encrypt property
    #[zbus(property)]
    fn encrypt(&self) -> zbus::Result<bool>;

    /// Es property
    #[zbus(property)]
    fn es(&self) -> zbus::Result<bool>;

    /// IcvLength property
    #[zbus(property)]
    fn icv_length(&self) -> zbus::Result<u8>;

    /// IncludeSci property
    #[zbus(property)]
    fn include_sci(&self) -> zbus::Result<bool>;

    /// Parent property
    #[zbus(property)]
    fn parent(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Protect property
    #[zbus(property)]
    fn protect(&self) -> zbus::Result<bool>;

    /// ReplayProtect property
    #[zbus(property)]
    fn replay_protect(&self) -> zbus::Result<bool>;

    /// Scb property
    #[zbus(property)]
    fn scb(&self) -> zbus::Result<bool>;

    /// Sci property
    #[zbus(property)]
    fn sci(&self) -> zbus::Result<u64>;

    /// Validation property
    #[zbus(property)]
    fn validation(&self) -> zbus::Result<String>;

    /// Window property
    #[zbus(property)]
    fn window(&self) -> zbus::Result<u32>;
}
