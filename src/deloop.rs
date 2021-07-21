
use anyhow::Result;
use configparser::ini::Ini;
use crate::{BASE_PATH,INI_FILE};
fn init() -> Result<()> {
    check(format!("{}\\{}", *BASE_PATH, *INI_FILE).as_str())?;

    if check(format!("{}\\{}_orig", *BASE_PATH, *INI_FILE).as_str()).is_ok() {
        backup(format!("{}\\{}", *BASE_PATH, *INI_FILE).as_str())?;
    };

    //Load Ini
    let mut config = Ini::new();
    // let map = config.load("resource/variables.ini")?;
    let map = match config.load("resource/variables.ini") {
        Ok(map) => map,
        Err(e) => {
            anyhow::bail!(e)
        }
    };

    dbg!(&map);


    // Toggle K:V
    Ok(())
}

fn check(path: &str) -> Result<()> {
    match std::path::Path::new(path).exists() {
        true => Ok(()),
        false => Err(anyhow::anyhow!("Could not locate ini file")),
    }
}

fn backup(path: &str) -> Result<()> {
    std::fs::copy(
        std::path::Path::new(path),
        std::path::Path::new(format!("{}_orig", path).as_str()),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use spectral::prelude::*;

    #[test]
    fn invalid_file_check() {
        assert_that!(check("")).is_err();
    }

    #[test]
    fn valid_file_check() {
        let temp = assert_fs::TempDir::new().unwrap();
        let ini_file = temp.child("variables.ini");
        ini_file
            .write_file(std::path::Path::new("resource/variables.ini"))
            .unwrap();
        assert_that!(check(ini_file.path().to_str().unwrap().as_ref())).is_ok();
        temp.close().unwrap();
    }

    #[test]
    fn valid_backup_create() {
        let temp = assert_fs::TempDir::new().unwrap();
        let ini_file = temp.child("variables.ini");
        ini_file
            .write_file(std::path::Path::new("resource/variables.ini"))
            .unwrap();
        assert_that!(backup(ini_file.path().to_str().unwrap().as_ref())).is_ok();
        assert_that!(check(
            format!("{}_orig", ini_file.path().to_str().unwrap()).as_str()
        ))
        .is_ok();
        temp.close().unwrap();
    }
    #[test]
    fn invalid_backup_create() {
        let temp = assert_fs::TempDir::new().unwrap();
        let ini_file = temp.child("variables.ini");
        ini_file
            .write_file(std::path::Path::new("resource/variables.ini"))
            .unwrap();
        assert_that!(backup(
            std::path::Path::new("ezt").to_str().unwrap().as_ref()
        ))
        .is_err();
        assert_that!(check(
            format!("{}_orig", ini_file.path().to_str().unwrap()).as_str()
        ))
        .is_err();
        temp.close().unwrap();
    }
}
