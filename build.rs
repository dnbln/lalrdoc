use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

fn main() {
    lalrpop::process_root().unwrap();
}
