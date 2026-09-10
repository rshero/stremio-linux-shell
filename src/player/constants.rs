pub const FLOAT_PROPERTIES: &[&str] = &[
    "time-pos",
    "duration",
    "volume",
    "speed",
    "sub-pos",
    "sub-scale",
    "sub-delay",
    "panscan",
    "demuxer-cache-time",
];

pub const INT_PROPERTIES: &[&str] = &["vid"];

pub const BOOL_PROPERTIES: &[&str] = &[
    "pause",
    "buffering",
    "seeking",
    "osc",
    "paused-for-cache",
    "input-default-bindings",
    "input-vo-keyboard",
    "keepaspect",
];

pub const NODE_PROPERTIES: &[&str] = &["metadata", "track-list", "video-params"];

pub const STRING_PROPERTIES: &[&str] = &[
    "path",
    "mpv-version",
    "ffmpeg-version",
    "hwdec",
    "vo",
    "sub-color",
    "sub-back-color",
    "sub-border-color",
    "sid",
    "aid",
    "mute",
    "media-title",
    "force-media-title",
    "sub-ass-override",
];
