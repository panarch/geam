//! Source type declarations and constructor projections owned by the process provider.
//!
//! These types are for typed host registrations. Ordinary provider functions use
//! the concrete authoring values in the parent module.

pub use crate::process::schema::{
    AbnormalReason, Down, DownSchema, ExitReason, ExitReasonSchema, KilledReason, NamedSubject,
    NormalReason, OrdinarySubject, Subject, SubjectSchema,
};
pub use crate::schema::{
    Monitor, MonitorSchema, Port, PortSchema, Selector, SelectorSchema, Timer, TimerSchema,
};

/// Constructor projection for a process monitor notification.
pub type ProcessDown = geam_core::HostCustomConstructorAt<
    Down,
    geam_core::HostCustomIndex0,
    crate::process::schema::ProcessDown,
>;
/// Constructor projection for a port monitor notification's source shape.
pub type PortDown = geam_core::HostCustomConstructorAt<
    Down,
    geam_core::HostCustomIndexNext<geam_core::HostCustomIndex0>,
    crate::process::schema::PortDown,
>;
