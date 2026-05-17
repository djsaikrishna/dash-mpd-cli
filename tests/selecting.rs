//! Tests for stream selection/filtering
//
// To run tests while enabling printing to stdout/stderr
//
//    cargo test --test selecting -- --show-output


pub mod common;
use std::env;
use ffprobe::ffprobe;
use file_format::FileFormat;
use predicates::prelude::*;
use assert_cmd::cargo::cargo_bin_cmd;
use assert_fs::{prelude::*, TempDir};
use common::{check_file_size_approx, check_media_duration, setup_logging};



// Check that we are able to display metainformation on a manifest with --simulate and -v
#[test]
fn test_select_introspect() {
    setup_logging();
    let mpd_url = "https://ftp.itec.aau.at/datasets/mmsys22/Skateboarding/4sec/multi-codecs-manifest.mpd";

    cargo_bin_cmd!()
        .args(["-v", "--simulate", mpd_url])
        .assert()
        .stdout(predicate::str::contains("Only simulating"))
        .stdout(predicate::str::contains("av01.0.08M.08"))
        .stdout(predicate::str::contains("1280x720"))
        .stdout(predicate::str::contains("avc1.64003c"))
        .stdout(predicate::str::contains("7680x4320"))
        .stdout(predicate::str::contains("hev1"))
        .stdout(predicate::str::contains("vvc1.1.L67.CQA"))
        .success();
}

#[test]
fn test_select_av1_small() {
    setup_logging();
    if env::var("CI").is_ok() {
        return;
    }
    let mpd_url = "https://ftp.itec.aau.at/datasets/mmsys22/Skateboarding/4sec/multi-codecs-manifest.mpd";
    
    // First check the smallest AV1 stream
    let tmpd = TempDir::new().unwrap()
        .into_persistent_if(env::var("TEST_PERSIST_FILES").is_ok());
    let out = tmpd.child("mmsys22-multiple-video-adaptations-av1.mp4");
    cargo_bin_cmd!()
        .args(["-v",
               "--quality", "worst",
               "--prefer-video-codecs", "av01",
               "-o", &out.to_string_lossy(), mpd_url])
        .assert()
        .success();
    check_file_size_approx(&out, 2_031_045);
    check_media_duration(&out, 236.0);
    let format = FileFormat::from_file(&out).unwrap();
    assert_eq!(format, FileFormat::Mpeg4Part14Video);
    let meta = ffprobe(out).unwrap();
    assert_eq!(meta.streams.len(), 1);
    let video = meta.streams.iter()
        .find(|s| s.codec_type.eq(&Some(String::from("video"))))
        .expect("finding video stream");
    assert!(video.codec_name.eq(&Some(String::from("av1"))));
    assert_eq!(video.width, Some(320));
}


#[test]
fn test_select_hev1_small() {
    setup_logging();
    if env::var("CI").is_ok() {
        return;
    }
    let mpd_url = "https://ftp.itec.aau.at/datasets/mmsys22/Skateboarding/4sec/multi-codecs-manifest.mpd";
    let tmpd = TempDir::new().unwrap()
        .into_persistent_if(env::var("TEST_PERSIST_FILES").is_ok());
    let out = tmpd.child("mmsys22-multiple-video-adaptations-hev1.mp4");
    cargo_bin_cmd!()
        .args(["-v",
               "--quality", "worst",
               "--prefer-video-codecs", "inexistent,hev1,vp09",
               "-o", &out.to_string_lossy(), mpd_url])
        .assert()
        .success();
    check_file_size_approx(&out, 3_601_579);
    check_media_duration(&out, 236.0);
    let format = FileFormat::from_file(&out).unwrap();
    assert_eq!(format, FileFormat::Mpeg4Part14Video);
    let meta = ffprobe(out).unwrap();
    assert_eq!(meta.streams.len(), 1);
    let video = meta.streams.iter()
        .find(|s| s.codec_type.eq(&Some(String::from("video"))))
        .expect("finding video stream");
    assert!(video.codec_name.eq(&Some(String::from("hevc"))));
    assert_eq!(video.width, Some(320));
}


#[test]
fn test_select_vvc_big() {
    setup_logging();
    if env::var("CI").is_ok() {
        return;
    }
    let mpd_url = "https://ftp.itec.aau.at/datasets/mmsys22/Skateboarding/4sec/multi-codecs-manifest.mpd";
    let tmpd = TempDir::new().unwrap()
        .into_persistent_if(env::var("TEST_PERSIST_FILES").is_ok());
    let out = tmpd.child("mmsys22-multiple-video-adaptations-vvc.mp4");
    cargo_bin_cmd!()
        .args(["-v",
               "--quality", "best",
               "--prefer-video-codecs", "vvc1,h264",
               "-o", &out.to_string_lossy(), mpd_url])
        .assert()
        .success();
    check_file_size_approx(&out, 827_435_116);
    check_media_duration(&out, 236.0);
    let format = FileFormat::from_file(&out).unwrap();
    assert_eq!(format, FileFormat::Mpeg4Part14Video);
    let meta = ffprobe(out).unwrap();
    assert_eq!(meta.streams.len(), 1);
    let video = meta.streams.iter()
        .find(|s| s.codec_type.eq(&Some(String::from("video"))))
        .expect("finding video stream");
    assert!(video.codec_name.eq(&Some(String::from("vvc"))));
    assert_eq!(video.width, Some(7680));
}


#[test]
fn test_select_id() {
    setup_logging();
    if env::var("CI").is_ok() {
        return;
    }
    let mpd_url = "https://ftp.itec.aau.at/datasets/mmsys22/Skateboarding/4sec/multi-codecs-manifest.mpd";
    let tmpd = TempDir::new().unwrap()
        .into_persistent_if(env::var("TEST_PERSIST_FILES").is_ok());
    let out = tmpd.child("mmsys22-multiple-video-adaptations-id.mp4");
    cargo_bin_cmd!()
        .args(["-v",
               "--quality", "worst",
               "--want-video-id", "34",
               "-o", &out.to_string_lossy(), mpd_url])
        .assert()
        .success();
    check_media_duration(&out, 236.0);
    let format = FileFormat::from_file(&out).unwrap();
    assert_eq!(format, FileFormat::Mpeg4Part14Video);
    let meta = ffprobe(out).unwrap();
    assert_eq!(meta.streams.len(), 1);
    let video = meta.streams.iter()
        .find(|s| s.codec_type.eq(&Some(String::from("video"))))
        .expect("finding video stream");
    assert!(video.codec_name.eq(&Some(String::from("hevc"))));
    assert_eq!(video.width, Some(640));
}

