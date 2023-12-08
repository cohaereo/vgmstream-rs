use core::slice;
use std::{
    ffi::CStr,
    io::{Cursor, Read, Seek},
    mem::transmute,
    ptr::null_mut,
};

use vgmstream_sys::{offv_t, STREAMFILE};

#[repr(C)]
pub struct MemoryStream {
    pub vt: STREAMFILE,

    size: usize,
    cur: Cursor<Vec<u8>>,
    name: Option<String>,
}

macro_rules! tms {
    ($v:expr) => {
        (*unsafe { (::std::mem::transmute::<*mut STREAMFILE, *mut MemoryStream>($v)) })
    };
}

impl MemoryStream {
    pub fn as_streamfile(&mut self) -> *mut STREAMFILE {
        unsafe { transmute(self as *mut Self) }
    }

    pub fn from_slice<S: AsRef<[u8]>>(s: S, name: Option<String>) -> MemoryStream {
        let data = s.as_ref().to_vec();

        MemoryStream {
            vt: STREAMFILE {
                read: Self::read,
                get_size: Self::get_size,
                get_offset: Self::get_offset,
                get_name: Self::get_name,
                open: Self::open,
                close: Self::close,
                stream_index: 0,
            },
            size: data.len(),
            cur: Cursor::new(data),
            name,
        }
    }

    unsafe extern "C" fn read(
        sf: *mut STREAMFILE,
        dst: *mut u8,
        offset: offv_t,
        length: usize,
    ) -> usize {
        let dst_slice = slice::from_raw_parts_mut(dst, length);
        tms!(sf)
            .cur
            .seek(std::io::SeekFrom::Start(offset as u64))
            .ok();
        tms!(sf).cur.read(dst_slice).unwrap_or_default()
    }

    unsafe extern "C" fn get_offset(sf: *mut STREAMFILE) -> offv_t {
        tms!(sf).cur.stream_position().unwrap() as i64
    }

    unsafe extern "C" fn get_size(sf: *mut STREAMFILE) -> usize {
        tms!(sf).size
    }

    unsafe extern "C" fn get_name(
        sf: *mut STREAMFILE,
        name: *mut ::std::os::raw::c_char,
        name_size: usize,
    ) {
        if let Some(fname) = &tms!(sf).name {
            name.copy_from(fname.as_ptr() as *const i8, name_size.min(fname.len()));
        }
    }

    unsafe extern "C" fn open(
        sf: *mut STREAMFILE,
        filename: *const ::std::os::raw::c_char,
        _buf_size: usize,
    ) -> *mut STREAMFILE {
        // cohae: vgmstream reopens the current file for no apparent reason???
        if let Some(fname) = &tms!(sf).name {
            if CStr::from_ptr(filename).to_string_lossy() == *fname {
                return sf;
            }
        }

        null_mut()
    }

    unsafe extern "C" fn close(_sf: *mut STREAMFILE) {}
}
