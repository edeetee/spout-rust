use std::{pin::Pin, ffi::CString};

// pub mod sender;
// pub mod utils;
// pub mod bridge;

use autocxx::prelude::*;

include_cpp! {
    #include "Spout.h"
    #include "SpoutUtils.h"

    safety!(unsafe_ffi)
    
    generate!("Spout")
    generate!("spoututils::OpenSpoutConsole")
}

pub struct SpoutSender{
    spout: Pin<Box<ffi::Spout>>
}

impl Default for SpoutSender {
    fn default() -> Self {
        Self::new("RustSpoutSender".into())
    }
}

impl SpoutSender {
    pub fn new(name: &str) -> Self {
        let spout = ffi::Spout::new().within_box();

        // let name = CString::new(name).unwrap();

        let mut me = SpoutSender{
            spout,
        };
        
        // unsafe {
        //     me.spout.as_mut().CreateSender(name.as_ptr(), 256.into(), 256.into(), 0.into());
        // }

        me.set_name(name);

        // dbg!(&me);

        // me.spout.as_mut().Set

        me
    }

    pub fn set_name(&mut self, name: &str) {
        let name = CString::new(name).unwrap();
        unsafe {
            self.spout.as_mut().SetSenderName(name.as_ptr());
        }
    }

    pub fn send_texture(&mut self, target_type: u32, texture_id: u32, width: u32, height: u32) {
        let success = self.spout.as_mut().SendTexture(texture_id.into(), target_type.into(), width.into(), height.into(), false, 0.into());
        debug_assert!(success);
    }
}

impl Drop for SpoutSender {
    fn drop(&mut self) {
        self.spout.as_mut().ReleaseSender();
    }
}