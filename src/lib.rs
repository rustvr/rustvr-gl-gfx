//  Copyright 2016 RustVR
//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate rustvr_core;
#[macro_use]
extern crate gfx; // !!! must be declared here due to #[macro_use]

pub mod terrain;

pub struct PluginGlGfx {
}

impl rustvr_core::Plugin for PluginGlGfx {
}

impl rustvr_core::PluginGl for PluginGlGfx {
    fn run_terrain(&self) {
        ::terrain::main()
    }
}

pub fn make_plugin() -> PluginGlGfx {
    PluginGlGfx {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
    #[test]
    fn terrain_main() {
        super::terrain::main();
    }
}
