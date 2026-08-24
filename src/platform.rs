use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegistryHive {
    CurrentUser,
    LocalMachine,
}

#[cfg(target_os = "windows")]
pub fn get_registry_dword(
    hive: RegistryHive,
    path: &str,
    name: &str,
) -> anyhow::Result<Option<u32>> {
    use std::io::ErrorKind;
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    use winreg::RegKey;

    let root = match hive {
        RegistryHive::CurrentUser => RegKey::predef(HKEY_CURRENT_USER),
        RegistryHive::LocalMachine => RegKey::predef(HKEY_LOCAL_MACHINE),
    };
    let key = match root.open_subkey(path) {
        Ok(key) => key,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    match key.get_value(name) {
        Ok(value) => Ok(Some(value)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.into()),
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_registry_dword(
    _hive: RegistryHive,
    _path: &str,
    _name: &str,
) -> anyhow::Result<Option<u32>> {
    anyhow::bail!("Windows 注册表仅能在 Windows 上读取")
}

#[cfg(target_os = "windows")]
pub fn delete_registry_value(hive: RegistryHive, path: &str, name: &str) -> anyhow::Result<()> {
    use std::io::ErrorKind;
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    use winreg::RegKey;

    let root = match hive {
        RegistryHive::CurrentUser => RegKey::predef(HKEY_CURRENT_USER),
        RegistryHive::LocalMachine => RegKey::predef(HKEY_LOCAL_MACHINE),
    };
    let key = match root.open_subkey_with_flags(path, winreg::enums::KEY_WRITE) {
        Ok(key) => key,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };
    match key.delete_value(name) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.into()),
    }
}

#[cfg(not(target_os = "windows"))]
pub fn delete_registry_value(_hive: RegistryHive, _path: &str, _name: &str) -> anyhow::Result<()> {
    anyhow::bail!("Windows 注册表仅能在 Windows 上修改")
}

#[cfg(target_os = "windows")]
pub fn set_registry_dword(
    hive: RegistryHive,
    path: &str,
    name: &str,
    value: u32,
) -> anyhow::Result<()> {
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    use winreg::RegKey;

    let root = match hive {
        RegistryHive::CurrentUser => RegKey::predef(HKEY_CURRENT_USER),
        RegistryHive::LocalMachine => RegKey::predef(HKEY_LOCAL_MACHINE),
    };
    let (key, _) = root.create_subkey(path)?;
    key.set_value(name, &value)?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn set_registry_dword(
    _hive: RegistryHive,
    _path: &str,
    _name: &str,
    _value: u32,
) -> anyhow::Result<()> {
    anyhow::bail!("Windows 注册表仅能在 Windows 上修改")
}

#[cfg(target_os = "windows")]
pub fn set_user_environment(name: &str, value: &str) -> anyhow::Result<()> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let root = RegKey::predef(HKEY_CURRENT_USER);
    let (environment, _) = root.create_subkey("Environment")?;
    environment.set_value(name, &value)?;
    std::env::set_var(name, value);
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn set_user_environment(_name: &str, _value: &str) -> anyhow::Result<()> {
    anyhow::bail!("Windows 用户环境变量仅能在 Windows 上修改")
}

#[cfg(target_os = "windows")]
pub fn prepend_user_path(path: &Path) -> anyhow::Result<bool> {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    let root = RegKey::predef(HKEY_CURRENT_USER);
    let (environment, _) = root.create_subkey("Environment")?;
    let current: String = environment.get_value("Path").unwrap_or_default();
    let candidate = path.to_string_lossy();
    let already_persisted = current
        .split(';')
        .any(|entry| entry.trim().eq_ignore_ascii_case(&candidate));
    if !already_persisted {
        let updated = if current.trim().is_empty() {
            candidate.clone().into_owned()
        } else {
            format!("{};{}", candidate, current)
        };
        environment.set_value("Path", &updated)?;
    }

    let mut process_paths: Vec<_> = std::env::var_os("PATH")
        .map(|value| std::env::split_paths(&value).collect())
        .unwrap_or_default();
    if !process_paths
        .iter()
        .any(|existing| existing.to_string_lossy().eq_ignore_ascii_case(&candidate))
    {
        process_paths.insert(0, path.to_path_buf());
        std::env::set_var("PATH", std::env::join_paths(process_paths)?);
    }

    Ok(!already_persisted)
}

#[cfg(not(target_os = "windows"))]
pub fn prepend_user_path(_path: &Path) -> anyhow::Result<bool> {
    anyhow::bail!("Windows 用户 PATH 仅能在 Windows 上修改")
}
