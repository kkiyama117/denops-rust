use std::{
    env,
    path::PathBuf,
    process::Command,
    fs::canonicalize,
};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(author, about)]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    base_dir: Option<PathBuf>,
    /// File name of output
    #[structopt(short, long, default_value = "index.js")]
    output: String,
}

fn main() {
    // use getopts::Options;
    let args: Vec<String> = env::args().collect();
    let Opt { base_dir, output } = Opt::from_iter_safe(args).unwrap();
    let path = match base_dir {
        Some(b) => {
            if b.is_absolute(){
                b
            }else{
                canonicalize(&b).unwrap()
            }
        },
        None => env::current_dir().unwrap(),
    };
    println!("base_dir: {:?}, output: {:?}", &path, &output);
    denops_build(&path, &output.as_str());
}

fn denops_build(base_dir: &PathBuf, out_file: &str) {
    let result = Command::new("wasm-pack")
        .args(&[
            "build",
            base_dir.to_str().unwrap(),
            "--target",
            "web",
            "--out-name",
            out_file,
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
