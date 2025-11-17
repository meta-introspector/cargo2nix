use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = Path::new("/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix");
    let submodules_dir = root_dir.join("submodules");

    println!("[workspace.dependencies]");

    let mut submodule_names: Vec<String> = fs::read_dir(&submodules_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                entry.file_name().into_string().ok()
            } else {
                None
            }
        })
        .collect();

    submodule_names.sort();

    for submodule in submodule_names {
        match submodule.as_str() {
            "serde" => {
                println!("serde = {{ path = \"./submodules/serde\" }}");
                println!("serde_derive = {{ path = \"./submodules/serde/serde_derive\" }}");
                println!("serde_core = {{ path = \"./submodules/serde/serde_core\" }}");
            },
            "time-rs" => {
                println!("time = {{ path = \"./submodules/time-rs/time\" }}");
                println!("time-core = {{ path = \"./submodules/time-rs/time-core\" }}");
                println!("time-macros = {{ path = \"./submodules/time-rs/time-macros\" }}");
            },
            "rand" => {
                println!("rand = {{ path = \"./submodules/rand\" }}");
                println!("rand08 = {{ path = \"./submodules/rand\" }}");
                println!("rand09 = {{ path = \"./submodules/rand\" }}");
            },
            _ => {
                println!("{} = {{ path = \"./submodules/{}\" }}", submodule, submodule);
            }
        }
    }

    Ok(())
}
