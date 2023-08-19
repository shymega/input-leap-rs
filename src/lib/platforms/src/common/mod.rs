// SPDX-FileCopyrightText: 2023 The Input-Leap Developers
//
// SPDX-License-Identifier: GPL-2.0-only

//! Stub module (TODO) for the `common` types.

/// `ScreenTrait` acts as a common specification of operations that can be executed on (TODO) a
/// `Screen` struct.
pub trait ScreenTrait {
    /// This function connects to the `Screen`.
    fn connect(&mut self) -> std::io::Result<()>;
    /// This function 'disables' a `Screen`.
    /// Basically, this means the screen remains connected, but can't be interacted with.
    fn disable(&mut self) -> std::io::Result<()>;
    /// This function disconnects from a `Screen`.
    fn disconnect(&mut self) -> std::io::Result<()>;
    /// This function 'enables' a `Screen`.
    /// Basically, this means the screen remains connected, and can be interacted with.
    fn enable(&mut self) -> std::io::Result<()>;
    /// This function force-disconnects from a `Screen`.
    fn force_disconnect(&mut self) -> std::io::Result<()>;
    /// This function returns the `human name` assigned to the `Screen`.
    fn get_human_name(&self) -> &str;
    /// This function returns the `Uuid` of the `Screen`.
    fn get_uuid(&self) -> uuid::Uuid;
    /// This function returns the Zeroconf name advertised of the `Screen`.
    fn get_zeroconf_name(&self) -> &str;
    /// This function returns a `bool` value, representing if a `Screen` is connected to the
    /// `Primary`.
    fn is_connected(&self) -> bool;
    /// This function returns a `bool` value, representing if a `Screen` is enabled.
    fn is_enabled(&self) -> bool;
    /// This function returns a `bool` value, representing if a `Screen` has control of the mouse/keyboard
    fn is_focused(&self) -> bool;
}
