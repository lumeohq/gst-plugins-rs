// Copyright (C) 2019 Ruben Gonzalez <rgonzalez@fluendo.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate glib;
#[macro_use]
extern crate gstreamer as gst;
#[macro_use]
extern crate gstreamer_audio as gst_audio;

mod claxondec;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    claxondec::register(plugin)
}

gst_plugin_define!(
    claxon,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    "MIT/X11",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);
