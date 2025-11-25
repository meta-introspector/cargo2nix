use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nix_dir = "/mnt/data/nix";
    let target_dir = "submodules";
    
    if !Path::new(nix_dir).exists() {
        println!("Source directory {} not found", nix_dir);
        return Ok(());
    }
    
    fs::create_dir_all(target_dir)?;
    
    for entry in fs::read_dir(nix_dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let source = entry.path();
        let target = Path::new(target_dir).join(&name);
        
        if entry.file_type()?.is_dir() && !name.to_string_lossy().starts_with('.') {
            println!("Importing: {:?} -> {:?}", source, target);
            
            if target.exists() {
                fs::remove_dir_all(&target)?;
            }
            
            copy_dir(&source, &target)?;
        }
    }
    
    println!("Import complete");
    Ok(())
}

fn copy_dir(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if entry.file_type()?.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}
