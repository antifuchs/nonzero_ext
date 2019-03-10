#![cfg(feature = "compiletest_rs")]

extern crate compiletest_rs as compiletest;
use std::fs::{read_dir, remove_file};
use std::path::PathBuf;

fn clean_rlibs(config: &compiletest_rs::Config) {
    if config.target_rustcflags.is_some() {
        for directory in config
            .target_rustcflags
            .as_ref()
            .unwrap()
            .split_whitespace()
        {
            if let Ok(mut entries) = read_dir(directory) {
                while let Some(Ok(entry)) = entries.next() {
                    let f = entry.file_name().clone().into_string().unwrap();
                    if f.ends_with(".rmeta") {
                        let prefix = &f[..f.len() - 5];
                        let _ = remove_file(entry.path());
                        if let Ok(mut entries) = read_dir(directory) {
                            while let Some(Ok(entry)) = entries.next() {
                                let f = entry.file_name().clone().into_string().unwrap();
                                if f.starts_with(prefix) && !f.ends_with(".rmeta") {
                                    let _ = remove_file(entry.path());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn run_mode(mode: &'static str) {
    let mut config = compiletest::Config::default().tempdir();
    let cfg_mode = mode.parse().expect("Invalid mode");

    config.mode = cfg_mode;
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    // config.target_rustcflags = Some("-L target/debug -L target/debug/deps".to_string());
    config.link_deps();
    config.clean_rmeta();
    clean_rlibs(&config);

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    // Don't run compile-fail tests under nightly: Error messages are unstable.
    #[cfg(feature = "stable")]
    run_mode("compile-fail");

    run_mode("run-pass");
}
