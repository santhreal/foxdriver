//! Firefox browser automation via WebDriver BiDi (rustenium).
//!
//! This crate provides a spawn-capable Firefox runtime: launch, drive,
//! evaluate, click, type, scroll, screenshot, cookies, and frames.
//! It is intentionally **independent** of `guise` (the stealth substrate)
//! so the fleet's layering stays one-way: guise may depend on foxdriver
//! for its `browser` feature, but foxdriver knows nothing about stealth
//! profiles, fingerprint bundles, or TLS impersonation.
//!
//! Consumers that need stealth should use `guise::browser` (which wraps
//! foxdriver with profile application) rather than using foxdriver directly.

#![forbid(unsafe_code)]

pub mod browser;
pub mod cookies;
pub mod dialog;
pub mod frame;
pub mod frame_graph;
pub mod network;
pub mod runtime;
pub mod sensors;

// Re-export the most common types at the crate root for ergonomics.
pub use browser::{
    launch_firefox, launch_firefox_self_managed, proxy_prefs, Element, EvaluationResult,
    FoxBrowserConfig, FrameId, FrameInfo, FrameTreeNode, Page, ProxyConfig, ProxyScheme,
    ScrollDirection,
};
pub use cookies::CapturedCookie;
pub use dialog::{CapturedDialog, CapturedDownload, DialogLog};
pub use frame_graph::{FrameGraph, FrameNode};
pub use network::{
    CapturedHeader, CapturedRequest, CapturedResponse, Filter, NetworkEntry, NetworkLog,
};
pub use runtime::drive_browser;
