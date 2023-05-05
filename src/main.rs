use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

use clap::Parser;
use walkdir::WalkDir;

/// renparkn: recursively rename files in provided directory by adding the parent directory
/// name while keeping the numbering (first number in file name is kept by default).
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to directory with files to be renamed
    dir_path: String,
    /// extract numbering after provided string (case sensitive)
    #[arg(short = 'a', long, value_name = "STRING", default_value_t = String::new())]
    num_after: String,
    /// show rename proposal but do not apply
    #[arg(short = 'n', long, action = clap::ArgAction::SetTrue)]
    dry_run: bool,
}

/// Returns vector with the paths of all files within provided directory.
fn list_files_recursively(dir_path: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir_path)
        .into_iter()
        .filter(Result::is_ok)
        .map(|e| e.unwrap().into_path())
        .filter(|e| e.is_file())
        .collect()
}

/// Returns number extracted from file name, first one by default or after specified string.
fn extract_numbering(file_name: &OsStr, num_after: &str) -> f32 {
    let binding = file_name.to_str().unwrap();
    let effective_file_name = match binding.split_once(num_after) {
        None => binding,
        Some(s) => s.1,
    };

    let mut found_digit = false;
    let mut num_str = String::new();
    for c in effective_file_name.chars() {
        if found_digit && c == '.' {
            if num_str.contains('.') {
                break;
            }
            num_str.push(c);
        } else if found_digit && !c.is_ascii_digit() {
            if num_str.ends_with('.') {
                num_str.pop();
            }
            break;
        } else if c.is_ascii_digit() {
            found_digit = true;
            num_str.push(c);
        }
    }
    num_str.parse::<f32>().unwrap_or(99.9)
}

/// Returns absolute path of the new name, generated by combining the parent name of the
/// file to be renamed with the extracted numbering and file extension.
fn generate_new_name(file_path: &Path, root_path: &Path, num_after: &str) -> Option<PathBuf> {
    let parent_name = file_path.parent()?.file_name()?.to_str()?;
    let num = extract_numbering(file_path.file_name()?, num_after);
    let ext = file_path.extension().unwrap_or(OsStr::new("")).to_str()?;
    Some(root_path.join(format!("{parent_name} {num}.{ext}")))
}

/// Prints provided map with renaming proposal as (`old_name` --> `new_name`)
fn print_rename_proposal(rename_pairs: HashMap<PathBuf, Option<PathBuf>>) {
    for (k, v) in rename_pairs {
        if let Some(val) = v {
            println!("{} --> {}", k.display(), val.display());
        } else {
            eprintln!("renparkn: warning: unable to rename \"{}\"", k.display());
            continue;
        }
    }
}

/// Applies rename operation defined in provided map with structure (`old_name`, `new_name`)
fn rename_files(rename_pairs: HashMap<PathBuf, Option<PathBuf>>) {
    for (k, v) in rename_pairs {
        if v.is_none() || fs::rename(&k, v.unwrap()).is_err() {
            eprintln!("renparkn: warning: unable to rename \"{}\"", k.display());
            continue;
        }
    }
}

fn main() {
    let args = Args::parse();

    let dir_path = Path::new(&args.dir_path);
    if !(dir_path.exists() && dir_path.is_dir()) {
        eprintln!("renparkn: error: directory not found");
        exit(1);
    }

    let rename_pairs: HashMap<PathBuf, Option<PathBuf>> = list_files_recursively(dir_path)
        .iter()
        .map(|p| (p.clone(), generate_new_name(p, dir_path, &args.num_after)))
        .collect();

    if args.dry_run {
        print_rename_proposal(rename_pairs);
    } else {
        rename_files(rename_pairs);
    }
}
