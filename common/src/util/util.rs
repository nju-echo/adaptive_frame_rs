use std::fs;
use std::path::Path;

//TODO:后续在看看
pub fn copy_directory(source_dir: &Path, dest_dir: &Path) -> std::io::Result<()> {
    if !dest_dir.exists() {
        fs::create_dir(dest_dir)?;
    }

    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let dest_file = dest_dir.join(entry.file_name());

        if entry.path().is_dir() {
            copy_directory(&entry.path(), &dest_file)?;
        } else {
            fs::copy(entry.path(), dest_file)?;
        }
    }
    Ok(())
}
