// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

#[cfg(feature = "manpage")]
use clap_mangen::Man;
use {
    anyhow::Result as AnyhowResult,
    std::env,
    vergen_gix::{CargoBuilder, Emitter, GixBuilder, RustcBuilder},
};
#[cfg(feature = "completions")]
use {
    clap::CommandFactory,
    clap_complete::generator::generate_to,
    clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh},
    std::{fs, path::Path},
};

#[cfg(feature = "completions")]
include!("../src/types.rs");

#[cfg(feature = "completions")]
include!("../src/cli.rs");

fn main() -> AnyhowResult<()> {
    println!("cargo:rustc-cfg=build");
    println!("cargo:rustc-check-cfg=cfg(build)");
    if let Ok(val) = env::var("AUTOTOOLS_DEPENDENCIES") {
        for dependency in val.split(' ') {
            println!("cargo:rerun-if-changed={dependency}");
        }
    }
    let mut builder = Emitter::default();
    builder.add_instructions(&CargoBuilder::all_cargo()?)?;
    // If passed a version from automake, use that instead of vergen's formatting
    if let Ok(val) = env::var("VERSION_FROM_AUTOTOOLS") {
        println!("cargo:rustc-env=VERGEN_GIT_DESCRIBE={val}");
        builder.add_instructions(&RustcBuilder::all_rustc()?)?;
    } else {
        builder.add_instructions(&GixBuilder::all_git()?)?;
    };
    builder.emit()?;
    #[cfg(feature = "manpage")]
    generate_manpage();
    #[cfg(feature = "completions")]
    generate_shell_completions();
    Ok(())
}

/// Generate man page
#[cfg(feature = "manpage")]
fn generate_manpage() {
    let out_dir = match env::var_os("OUT_DIR") {
        None => return,
        Some(out_dir) => out_dir,
    };
    let manpage_dir = Path::new(&out_dir);
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
    fs::write(manpage_dir.join(format!("{bin_name}.1")), buffer)
        .expect("Unable to write manepage to file");
}

/// Generate shell completion files from CLI interface
#[cfg(feature = "completions")]
fn generate_shell_completions() {
    let out_dir = match env::var_os("OUT_DIR") {
        None => return,
        Some(out_dir) => out_dir,
    };
    let completions_dir = Path::new(&out_dir).join("completions");
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
