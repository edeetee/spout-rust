use self::ffi::Spout;

#[cxx::bridge]
pub mod ffi {
    struct Spout {
        m_AdapterName: [u8; 256],
        m_bAdapt: bool
    }

    unsafe extern "C++" {
        include!("spout_rust/SpoutGL/Spout.h");

        pub type Spout;

        pub fn SetReceiverName(sendername: String) -> u32;
        pub fn GetSenderWidth() -> u32;
        pub fn GetSenderHeight() -> u32;
        pub fn IsFrameNew() -> bool;
    }
}

impl Spout{
    fn new() -> Self {
        Spout{
            m_AdapterName: [0; 256],
            m_bAdapt: false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let spout = Spout::new();
        let sender_name = "test".to_string();
        let result = unsafe { spout.SetReceiverName(sender_name) };
        assert_eq!(result, 0);
    }
}