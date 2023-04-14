use rfd::AsyncFileDialog;

pub async fn open_file() -> Option<String> {
    let file = AsyncFileDialog::new()
        .add_filter(
            "Video Files",
            &[
                "3g2", "3gp", "3gp2", "3gpp", "amrec", "amv", "asf", "avi", "bik", "crf", "dav",
                "divx", "drc", "dv", "dvr-ms", "evo", "f4v", "flv", "gvi", "gxf", "iso", "m1v",
                "m2v", "m2t", "m2ts", "m4v", "mkv", "mov", "mp2", "mp2v", "mp4", "mp4v", "mpe",
                "mpeg", "mpeg1", "mpeg2", "mpeg4", "mpg", "mpv2", "mts", "mtv", "mxf", "mxg",
                "nsv", "nuv", "ogg", "ogm", "ogv", "ogx", "ps", "rec", "rm", "rmvb", "rpl", "thp",
                "tod", "ts", "tts", "txd", "vob", "vro", "webm", "wm", "wmv", "wtv", "xesc",
            ],
        )
        .set_directory("/")
        .pick_file()
        .await;
    if let Some(file) = file {
        Some(format!(
            "file:///{}",
            file.path().to_str().unwrap().to_string()
        ))
    } else {
        None
    }
}
