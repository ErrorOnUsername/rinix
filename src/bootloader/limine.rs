#[allow(unused)]
#[link_section = ".stivale2hdr"]
static STIVALE2_HEADER: Stv2Header = Stv2Header {
    entry_point: 0,
    stack: KERNEL_STACK.as_ptr_range().end,
    flags: 0,
    tags: 0,
};

static KERNEL_STACK: [u8; 32768] = [0; 32768];

#[allow(dead_code)]
extern "C" fn boot_entry( _stv2_struct: usize ) -> !
{
    loop { }
}

#[repr(C, packed)]
struct Stv2Header {
    entry_point: usize,
    stack: *const u8,
    flags: usize,
    tags: usize,
}

unsafe impl Send for Stv2Header { }
unsafe impl Sync for Stv2Header { }

