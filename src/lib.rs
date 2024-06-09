use std::mem::zeroed;

use info::VgmstreamInfo;
use reader::MemoryStream;
use vgmstream_sys::{vgmstream_get_samples, vgmstream_set_log_stdout};

pub mod enums;
pub mod info;
pub mod reader;

pub fn read_file_to_samples(
    data: &[u8],
    filename: Option<String>,
) -> anyhow::Result<(Vec<i16>, VgmstreamInfo)> {
    unsafe {
        vgmstream_set_log_stdout(100);
        let mut fs = MemoryStream::from_slice(data, filename);

        let vg = vgmstream_sys::init_vgmstream_from_STREAMFILE(fs.as_streamfile());

        if vg.is_null() {
            anyhow::bail!("Failed to init vgmstream");
        }

        let mut desc = zeroed();
        vgmstream_sys::describe_vgmstream_info(vg, &mut desc);

        // Get sample count and render the samples
        let sample_count = vgmstream_get_samples(vg) as usize;

        let mut sample_buffer = vec![0i16; 32767 * desc.channels as usize];
        let mut final_buffer = vec![];

        for i in (0..sample_count).step_by(32767) {
            let to_get = if (i + 32767) > sample_count {
                sample_count - i
            } else {
                32767
            };

            let samples_done =
                vgmstream_sys::render_vgmstream(sample_buffer.as_mut_ptr(), to_get as i32, vg);

            final_buffer.extend(&sample_buffer[..samples_done as usize * desc.channels as usize]);
        }

        vgmstream_sys::close_vgmstream(vg);

        Ok((final_buffer, desc.into()))
    }
}

pub fn read_file_info(data: &[u8], filename: Option<String>) -> anyhow::Result<VgmstreamInfo> {
    unsafe {
        vgmstream_set_log_stdout(100);
        let mut fs = MemoryStream::from_slice(data, filename);

        let vg = vgmstream_sys::init_vgmstream_from_STREAMFILE(fs.as_streamfile());

        if vg.is_null() {
            anyhow::bail!("Failed to init vgmstream");
        }

        let mut desc = zeroed();
        vgmstream_sys::describe_vgmstream_info(vg, &mut desc);

        vgmstream_sys::close_vgmstream(vg);

        Ok(desc.into())
    }
}
