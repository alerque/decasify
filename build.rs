#[cfg(feature = "completions")]
use clap::CommandFactory;
#[cfg(feature = "completions")]
use clap_complete::generator::generate_to;
#[cfg(feature = "completions")]
use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};
#[cfg(feature = "manpage")]
use clap_mangen::Man;
use std::env;
#[cfg(feature = "completions")]
use std::{fs, path};
use vergen::{vergen, Config};

#[cfg(feature = "completions")]
include!("src/cli.rs");

fn main() {
    let mut flags = Config::default();
    // If passed a version, use that instead of vergen's formatting
    if let Ok(val) = env::var("DECASIFY_VERSION") {
        *flags.git_mut().semver_mut() = false;
        println!("cargo:rustc-env=VERGEN_GIT_SEMVER={}", val)
    };
    // vergen(flags).expect("Unable to generate the cargo keys!");
    // Try to output flags based on Git repo, but if that fails turn off Git features and try again
    // with just cargo generated version info
    if vergen(flags).is_err() {
        let mut flags = Config::default();
        *flags.git_mut().enabled_mut() = false;
        vergen(flags).expect("Unable to generate the cargo keys!");
    }
    #[cfg(feature = "completions")]
    generate_manpage();
    generate_shell_completions();
}

/// Generate man page
#[cfg(feature = "manpage")]
fn generate_manpage() {
    let out_dir = match env::var_os("OUT_DIR") {
        None => return,
        Some(out_dir) => out_dir,
    };
    let manpage_dir = path::Path::new(&out_dir);
    fs::create_dir_all(manpage_dir).expect("Unable to create directory for generated manpages");
    let app = Cli::command();
    let bin_name: &str = app
        .get_bin_name()
        .expect("Could not retrieve bin-name from generated Clap app");
    let app = Cli::command();
    let man = Man::new(app);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)
        .expect("Unable to render man page to UTF-8 string");
    fs::write(manpage_dir.join(format!("{}.1", bin_name)), buffer)
        .expect("Unable to write manepage to file");
}

/// Generate shell completion files from CLI interface
#[cfg(feature = "completions")]
fn generate_shell_completions() {
    let out_dir = match env::var_os("OUT_DIR") {
        None => return,
        Some(out_dir) => out_dir,
    };
    let completions_dir = path::Path::new(&out_dir).join("completions");
    fs::create_dir_all(&completions_dir)
        .expect("Could not create directory in which to place completions");
    let app = Cli::command();
    let bin_name: &str = app
        .get_bin_name()
        .expect("Could not retrieve bin-name from generated Clap app");
    let mut app = Cli::command();
    #[cfg(feature = "bash")]
    generate_to(Bash, &mut app, bin_name, &completions_dir)
        .expect("Unable to generate bash completions");
    #[cfg(feature = "elvish")]
    generate_to(Elvish, &mut app, bin_name, &completions_dir)
        .expect("Unable to generate elvish completions");
    #[cfg(feature = "fish")]
    generate_to(Fish, &mut app, bin_name, &completions_dir)
        .expect("Unable to generate fish completions");
    #[cfg(feature = "powershell")]
    generate_to(PowerShell, &mut app, bin_name, &completions_dir)
        .expect("Unable to generate powershell completions");
    #[cfg(feature = "zsh")]
    generate_to(Zsh, &mut app, bin_name, &completions_dir)
        .expect("Unable to generate zsh completions");
}
