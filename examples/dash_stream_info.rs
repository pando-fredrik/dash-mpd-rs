// dash_stream_info.rs
//
// Run with  `cargo run --example dash_stream_info <URL>`
//
// Example URL: http://ftp.itec.aau.at/datasets/mmsys12/ElephantsDream/MPDs/ElephantsDreamNonSeg_6s_isoffmain_DIS_23009_1_v_2_1c2_2011_08_30.mpd


use std::time::Duration;
use dash_mpd::parse;
use dash_mpd::{MPD, is_audio_adaptation, is_video_adaptation};
use clap::Arg;


fn main() {
    let matches = clap::App::new("dash_stream_info")
        .about("Show codec and bandwith for audio and video streams specified in a DASH MPD")
        .arg(Arg::with_name("url")
             .takes_value(true)
             .value_name("URL")
             .index(1)
             .required(true))
        .get_matches();
    let url = matches.value_of("url").unwrap();
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::new(30, 0))
        .gzip(true)
        .build()
        .expect("Couldn't create reqwest HTTP client");
    let xml = client.get(url)
        .header("Accept", "application/dash+xml")
        .send()
        .expect("requesting DASH MPD")
        .text()
        .expect("fetching MPD content");
    let mpd: MPD = parse(&xml).expect("parsing MPD content");
    let period = &mpd.periods[0];
    let unspecified = "<unspecified>".to_string();
    // Show audio tracks with the codecs and available bitrates. Note that MPD manifests used by
    // commercial streaming services often include separate audio tracks for the different dubbed
    // languages available, but audio may also be included with the video track.
    if let Some(audio_adaptation) = period.adaptations.as_ref().and_then(|a| a.iter().find(is_audio_adaptation)) {
        for representation in audio_adaptation.representations.iter() {
            // Here we see some of the fun of dealing with the MPD format: the codec can be
            // specified on the <Representation> element, or on the relevant <AdaptationSet>
            // element, or not at all.
            let codec = representation.codecs.as_ref()
                .unwrap_or(audio_adaptation.codecs.as_ref()
                           .unwrap_or(&unspecified));
            println!("Audio stream with codec {}, bandwidth {}", codec, representation.bandwidth);
        }
    }

    // Show video tracks with the codecs and available bitrates
    if let Some(video_adaptation) = period.adaptations.as_ref().and_then(|a| a.iter().find(is_video_adaptation)) {
        for representation in video_adaptation.representations.iter() {
            let codec = representation.codecs.as_ref()
                .unwrap_or(video_adaptation.codecs.as_ref()
                           .unwrap_or(&unspecified));
            println!("Video stream with codec {}, bandwidth {}", codec, representation.bandwidth);
        }
    }
}
