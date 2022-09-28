use autocxx::include_cpp;

include_cpp! {
    #include "Spout.h"
    #include "SpoutUtils.h"

    safety!(unsafe_ffi)
    
    generate!("Spout")
    generate!("spoututils::OpenSpoutConsole")
}

pub use ffi::*;