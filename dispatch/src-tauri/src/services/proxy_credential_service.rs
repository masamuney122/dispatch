const SERVICE: &str = "com.yapikredi.dispatch.proxy";
const ACCOUNT: &str = "manual-proxy";

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn entry() -> Result<keyring::Entry, String> {
    keyring::Entry::new(SERVICE, ACCOUNT)
        .map_err(|error| format!("Proxy credential store could not be opened: {error}"))
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn load_password() -> Result<Option<String>, String> {
    match entry()?.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(error) => Err(format!(
            "Proxy password could not be read securely: {error}"
        )),
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn load_password() -> Result<Option<String>, String> {
    Ok(None)
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn store_password(password: &str) -> Result<(), String> {
    entry()?
        .set_password(password)
        .map_err(|error| format!("Proxy password could not be stored securely: {error}"))
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn store_password(_password: &str) -> Result<(), String> {
    Err("Secure proxy credential storage is not available on this platform.".into())
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn delete_password() -> Result<(), String> {
    match entry()?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(error) => Err(format!("Proxy password could not be removed: {error}")),
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn delete_password() -> Result<(), String> {
    Ok(())
}
