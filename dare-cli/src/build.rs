use std::{env, path::Path};

use clap::IntoApp;
use clap_complete::generate_to;

mod command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let mut cmd = command::Args::command();

    #[cfg(target_os = "linux")]
    {
        use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};

        generate_to(Bash, &mut cmd, "dare", &out_dir).unwrap();
        generate_to(Elvish, &mut cmd, "dare", &out_dir).unwrap();
        generate_to(Fish, &mut cmd, "dare", &out_dir).unwrap();
        generate_to(PowerShell, &mut cmd, "dare", &out_dir).unwrap();
        generate_to(Zsh, &mut cmd, "dare", &out_dir).unwrap();
    }

    #[cfg(target_os = "windows")]
    {
        use clap_complete::shells::PowerShell;

        generate_to(PowerShell, &mut cmd, "dare", &out_dir).unwrap();
    }

    #[cfg(target_os = "macos")]
    {
        use clap_complete::shells::{Bash, Elvish, Fish, PowerShell, Zsh};

        generate_to(Bash, &mut cmd, "dare", &out_dir).unwrap();
        generate_to(Elvish, &mut cmd, "dare", &out_dir).unwrap();
        generate_to(Fish, &mut cmd, "dare", &out_dir).unwrap();
        generate_to(PowerShell, &mut cmd, "dare", &out_dir).unwrap();
        generate_to(Zsh, &mut cmd, "dare", &out_dir).unwrap();
    }
}
