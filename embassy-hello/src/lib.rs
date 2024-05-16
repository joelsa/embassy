#![cfg_attr(not(any(feature = "std", feature = "wasm", test)), no_std)]
#![allow(async_fn_in_trait)]

use log::info;

pub struct Hello;

impl Hello {

    pub fn say_hello() {
        info!("Hello!");
    }
}
