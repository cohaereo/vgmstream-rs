use std::ffi::CStr;

use vgmstream_sys::vgmstream_info;

#[derive(Debug, Clone)]
pub struct VgmstreamInfo {
    pub sample_rate: i32,
    pub channels: i32,
    pub mixing_info: VgmstreamMixingInfo,
    pub channel_layout: i32,
    pub loop_info: VgmstreamLoopInfo,
    pub num_samples: usize,
    pub encoding: String,
    pub layout: String,
    pub interleave_info: VgmstreamInterleaveInfo,
    pub frame_size: i32,
    pub metadata: String,
    pub bitrate: i32,
    pub stream_info: VgmstreamStreamInfo,
}

impl From<vgmstream_info> for VgmstreamInfo {
    fn from(desc: vgmstream_info) -> Self {
        let encoding =
            unsafe { CStr::from_ptr(desc.encoding.as_ptr()).to_string_lossy() }.to_string();
        let layout = unsafe { CStr::from_ptr(desc.layout.as_ptr()).to_string_lossy() }.to_string();
        let metadata =
            unsafe { CStr::from_ptr(desc.metadata.as_ptr()).to_string_lossy() }.to_string();
        let stream_name =
            unsafe { CStr::from_ptr(desc.stream_info.name.as_ptr()).to_string_lossy() }.to_string();

        Self {
            sample_rate: desc.sample_rate,
            channels: desc.channels,
            mixing_info: VgmstreamMixingInfo {
                input_channels: desc.mixing_info.input_channels,
                output_channels: desc.mixing_info.output_channels,
            },
            channel_layout: desc.channel_layout,
            loop_info: VgmstreamLoopInfo {
                start: desc.loop_info.start,
                end: desc.loop_info.end,
            },
            num_samples: desc.num_samples,
            encoding,
            layout,
            interleave_info: VgmstreamInterleaveInfo {
                value: desc.interleave_info.value,
                first_block: desc.interleave_info.first_block,
                last_block: desc.interleave_info.last_block,
            },
            frame_size: desc.frame_size,
            metadata,
            bitrate: desc.bitrate,
            stream_info: VgmstreamStreamInfo {
                current: desc.stream_info.current,
                total: desc.stream_info.total,
                name: stream_name,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct VgmstreamMixingInfo {
    pub input_channels: i32,
    pub output_channels: i32,
}

#[derive(Debug, Clone)]
pub struct VgmstreamLoopInfo {
    pub start: i32,
    pub end: i32,
}

#[derive(Debug, Clone)]
pub struct VgmstreamInterleaveInfo {
    pub value: i32,
    pub first_block: i32,
    pub last_block: i32,
}

#[derive(Debug, Clone)]
pub struct VgmstreamStreamInfo {
    pub current: i32,
    pub total: i32,
    pub name: String,
}
