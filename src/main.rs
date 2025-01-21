// sota staircase SideStepper
// a fast .gitignore-respecting large file finder
//
// Copyright (c) 2025 mark joshwel <mark@joshwel.co>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR
// IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

const SOTA_SIDESTEP_CHUNK_SIZE: u16 = 16;
const SOTA_SIDESTEP_MAX_WORKERS: u16 = 4;
const SOTA_SIDESTEP_LARGE_FILE_SIZE: u64 = 100000000; // 100mb

#[derive(Debug)]
struct Behaviour {
    repo_dir_path: PathBuf,
    repo_sotaignore_path: PathBuf,
    // parallel: bool,
    // chunk_size: u16,
    // max_workers: u16,
    large_file_size: u64,
    search_here: bool,
    plumbing: bool,
}

fn cli_get_behaviour() -> Result<Behaviour, Box<dyn Error>> {
    // get environment variables
    // let parallel: bool = 'get_parallel: {
    //     // future me move this to a higher block if we ever need args
    //     // anywhere else also what the hell, labeled blocks?
    //     // huh -- the community seems wishy-washy on it,
    //     // but this seems like a harmless use of em
    //     let args: Vec<String> = env::args().collect();
    //     if env::var("SOTA_SIDESTEP_PARALLEL").is_ok() {
    //         break 'get_parallel true;
    //     }
    //     if args.iter().any(|arg| arg == "--parallel") {
    //         break 'get_parallel true;
    //     }
    //     false
    // };
    // let chunk_size: u16 = match env::var("SOTA_SIDESTEP_CHUNK_SIZE") {
    //     Ok(val) => val.parse::<u16>().unwrap_or(SOTA_SIDESTEP_CHUNK_SIZE),
    //     Err(_) => SOTA_SIDESTEP_CHUNK_SIZE,
    // };
    // let max_workers: u16 = match env::var("SOTA_SIDESTEP_MAX_WORKERS") {
    //     Ok(val) => val.parse::<u16>().unwrap_or(SOTA_SIDESTEP_MAX_WORKERS),
    //     Err(_) => SOTA_SIDESTEP_MAX_WORKERS,
    // };
    let large_file_size: u64 = match env::var("SOTA_SIDESTEP_LARGE_FILE_SIZE") {
        Ok(val) => val.parse::<u64>().unwrap_or(SOTA_SIDESTEP_LARGE_FILE_SIZE),
        Err(_) => SOTA_SIDESTEP_LARGE_FILE_SIZE,
    };

    // look through args and see if the '--search-here' or '--plumbing' flags are present
    let mut search_here: bool = false;
    let mut plumbing: bool = false;
    for arg in env::args() {
        if arg == "--search-here" {
            search_here = true;
        }
        if arg == "--plumbing" {
            plumbing = true;
        }
    }

    let current_dir = env::current_dir().unwrap();

    // if we're searching here anywas, return early using the current dir
    if search_here {
        return Ok(Behaviour {
            repo_dir_path: PathBuf::from(&current_dir),
            repo_sotaignore_path: PathBuf::from(current_dir.join(".sotaignore")),
            large_file_size,
            search_here,
            plumbing,
        });
    }

    // else, find the repo dir
    // (go through each parent dir until one of them has a .git directory in it)
    let mut dir = current_dir.as_path();
    let mut possible_repo_dir_path: Option<&Path> = None;
    while dir.components().count() > 1 {
        // check if there's a .git directory nearby
        if dir.join(".git/").try_exists().ok() == Some(true) {
            possible_repo_dir_path = Option::from(dir);
            break;
        }

        // iterate down!
        if let Some(parent) = dir.parent() {
            dir = parent;
        } else {
            break;
        }
    }
    if possible_repo_dir_path.is_none() {
        return Err("could not find a .git repository in the current or parent directories".into());
    }
    let repo_dir_path = possible_repo_dir_path.unwrap();
    Ok(Behaviour {
        repo_dir_path: PathBuf::from(repo_dir_path),
        repo_sotaignore_path: PathBuf::from(repo_dir_path.join(".sotaignore")),
        large_file_size,
        search_here,
        plumbing,
    })
}

fn main() {
    eprintln!("\nsota staircase SideStepper v5 (i3/a4)");
    let behaviour = {
        let behaviour = cli_get_behaviour();
        // huh. pattern matching consumes the variable, so we ref (&) it. damn.
        if let Err(e) = &behaviour {
            eprintln!("critical error: {}\n", e);
            std::process::exit(1);
        }
        behaviour.unwrap()
    };
    eprintln!(
        "   repo root : {}\n .sotaignore : {}\n",
        behaviour.repo_dir_path.to_str().unwrap(),
        {
            if behaviour.plumbing {
                "(stdout)".into()
            } else {
                format!(
                    "{} ({})",
                    behaviour.repo_sotaignore_path.to_str().unwrap(),
                    {
                        if behaviour.repo_sotaignore_path.try_exists().ok() == Some(true) {
                            "exists"
                        } else {
                            "non-existent"
                        }
                    }
                )
            }
        },
    );
}
