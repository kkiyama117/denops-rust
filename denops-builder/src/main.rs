use std::ffi::OsStr;
use std::{env, path::PathBuf, process::Command};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(author, about)]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    base_dir: Option<PathBuf>,
    /// Path to the config
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() {
    // use getopts::Options;
    let args: Vec<String> = env::args().collect();
    let Opt { base_dir, output } = Opt::from_iter_safe(args).unwrap();
    let path = match base_dir {
        Some(b) => b,
        None => env::current_dir().unwrap(),
    };
    let output = match output {
        Some(o) => o,
        None => env::current_dir().unwrap().join("index.js"),
    };
    println!("{:?},{:?}", path, output);
    // denops_build(&path, &output);
}

fn denops_build(base_dir: &PathBuf, out_file: &PathBuf) {
    let result = Command::new("wasm-pack")
        .args(&[
            "build",
            "--target",
            "web",
            "--out-name",
            out_file.to_str().unwrap(),
        ])
        .output()
        .expect("failed to build by wasm-pack");
    let result = String::from_utf8(result.stderr).unwrap();
    println!("{}", result);
    let result2 =
    Command::new("sed")
        .args(&["-i", "-e", "s#input = fetch(input);#if (typeof Deno !== 'undefined') input = new WebAssembly.Module(await Deno.readFile(new URL(input).pathname));#", ])
        .arg(&base_dir.join("pkg").join(out_file.to_owned()))
        .output()
        .expect("failed to run sed");
    let result2 = String::from_utf8(result2.stderr).unwrap();
    println!("{}", result2);
}
