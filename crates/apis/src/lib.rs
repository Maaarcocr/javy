//! JS APIs for Javy.
//!
//! This crate provides JS APIs you can add to Javy.
//!
//! Example usage:
//! ```
//! # use anyhow::{anyhow, Error, Result};
//! use javy::{quickjs::JSValue, Runtime};
//! use javy_apis::RuntimeExt;
//!
//! let runtime = Runtime::new_with_defaults()?;
//! let context = runtime.context();
//! context.global_object()?.set_property(
//!    "print",
//!    context.wrap_callback(move |_ctx, _this, args| {
//!        let str = args
//!            .first()
//!            .ok_or(anyhow!("Need to pass an argument"))?
//!            .to_string();
//!        println!("{str}");
//!        Ok(JSValue::Undefined)
//!    })?,
//! )?;
//! context.eval_global("hello.js", "print('hello!');")?;
//! # Ok::<(), Error>(())
//! ```
//!
//! If you want to customize the runtime or the APIs, you can use the
//! [`Runtime::new_with_apis`] method instead to provide a [`javy::Config`]
//! for the underlying [`Runtime`] or an [`APIConfig`] for the APIs.
//!
//! ## Features
//! * `console` - registers an implementation of the `console` API
//! * `text_encoding` - registers implementations of `TextEncoder` and `TextDecoder`
//! * `stream_io` - registers implementations of `Javy.IO.readSync` and `Javy.IO.writeSync`

use anyhow::Result;
use javy::Runtime;

pub use api_config::APIConfig;
#[cfg(feature = "console")]
pub use console::LogStream;
pub use runtime_ext::RuntimeExt;

mod api_config;
#[cfg(feature = "console")]
mod console;
mod runtime_ext;
#[cfg(feature = "stream_io")]
mod stream_io;
#[cfg(feature = "text_encoding")]
mod text_encoding;

pub(crate) trait JSApiSet {
    fn register(&self, runtime: &Runtime, config: &APIConfig) -> Result<()>;
}

/// Adds enabled JS APIs to the provided [`Runtime`].
///
/// ## Example
/// ```
/// # use anyhow::Error;
/// # use javy::Runtime;
/// # use javy_apis::APIConfig;
/// let runtime = Runtime::default();
/// javy_apis::add_to_runtime(&runtime, APIConfig::default())?;
/// # Ok::<(), Error>(())
/// ```
pub fn add_to_runtime(runtime: &Runtime, config: APIConfig) -> Result<()> {
    #[cfg(feature = "console")]
    console::Console::new().register(runtime, &config)?;
    #[cfg(feature = "stream_io")]
    stream_io::StreamIO.register(runtime, &config)?;
    #[cfg(feature = "text_encoding")]
    text_encoding::TextEncoding.register(runtime, &config)?;
    Ok(())
}