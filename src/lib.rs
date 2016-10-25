//  Copyright 2016 RustVR
//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_use]
extern crate gfx; // !!! must be declared here due to #[macro_use]

pub mod terrain;

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
