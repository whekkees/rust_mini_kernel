
#[repr(C)]
struct DeviceInfo {
    id : u32,
    category : u32,
    name : *const u8,
}

unsafe extern "C" {
    fn device_register();

    
}