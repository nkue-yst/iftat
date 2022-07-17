use clap::{Command, IntoApp};
use clap_complete::Shell;
use std::fs::File;
use std::path::Path;

include!("src/cli.rs");

fn generate(s: Shell, app: &mut Command, outdir: &Path, file: &str) {
    let destfile = outdir.join(file);
    println!("dest: {}", destfile.display());
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    let mut dest = File::create(destfile).unwrap();

    clap_complete::generate(s, app, "iftat", &mut dest);
}

fn main() {
    let mut app = Options::command();
    app.set_bin_name("iftat");

    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/completions/");

    generate(Shell::Bash, &mut app, &outdir, "bash/iftat");
    generate(Shell::Elvish, &mut app, &outdir, "elvish/iftat");
    generate(Shell::Fish, &mut app, &outdir, "fish/iftat");
    generate(Shell::PowerShell, &mut app, &outdir, "powershell/iftat");
    generate(Shell::Zsh, &mut app, &outdir, "zsh/_iftat");
}
