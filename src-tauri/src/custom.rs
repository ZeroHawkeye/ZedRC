use winreg::{
    enums::{HKEY_CLASSES_ROOT, HKEY_CURRENT_USER},
    RegKey,
};

#[tauri::command]
pub fn reg_right(path: &str) -> Result<(), String> {
    let hcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let hcu = RegKey::predef(HKEY_CURRENT_USER);

    // Register right-click menu for empty area in a folder
    let (reg_name, _) = hcr
        .create_subkey("Directory\\Background\\shell\\Zed")
        .map_err(|e| format!("{:?}", e))?;
    reg_name
        .set_value("Icon", &path)
        .map_err(|e| format!("{:?}", e))?;
    reg_name
        .set_value("", &"Open in Zed")
        .map_err(|e| format!("{:?}", e))?;
    let (reg_comm, _) = hcr
        .create_subkey("Directory\\Background\\shell\\Zed\\command")
        .map_err(|e| format!("{:?}", e))?;
    reg_comm
        .set_value("", &format!("{} \"%V\"", path))
        .map_err(|e| format!("{:?}", e))?;

    // Register right-click menu for folders
    let (reg_folder_name, _) = hcu
        .create_subkey("SOFTWARE\\Classes\\Directory\\shell\\Zed")
        .map_err(|e| format!("{:?}", e))?;
    reg_folder_name
        .set_value("Icon", &path)
        .map_err(|e| format!("{:?}", e))?;
    reg_folder_name
        .set_value("", &"Open in Zed")
        .map_err(|e| format!("{:?}", e))?;
    let (reg_folder_comm, _) = hcu
        .create_subkey("SOFTWARE\\Classes\\Directory\\shell\\Zed\\command")
        .map_err(|e| format!("{:?}", e))?;
    reg_folder_comm
        .set_value("", &format!("{} \"%V\"", path))
        .map_err(|e| format!("{:?}", e))?;

    // Register right-click menu for files
    let (reg_file_name, _) = hcu
        .create_subkey("SOFTWARE\\Classes\\*\\shell\\Zed")
        .map_err(|e| format!("{:?}", e))?;
    reg_file_name
        .set_value("Icon", &path)
        .map_err(|e| format!("{:?}", e))?;
    reg_file_name
        .set_value("", &"Open in Zed")
        .map_err(|e| format!("{:?}", e))?;
    let (reg_file_comm, _) = hcu
        .create_subkey("SOFTWARE\\Classes\\*\\shell\\Zed\\command")
        .map_err(|e| format!("{:?}", e))?;
    reg_file_comm
        .set_value("", &format!("{} \"%V\"", path))
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

#[tauri::command]
pub fn delete_right() -> Result<(), String> {
    let hcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let hcu = RegKey::predef(HKEY_CURRENT_USER);

    // Delete folder right-click menu
    hcr.delete_subkey(r"Directory\Background\shell\Zed\command")
        .map_err(|e| format!("{:?}", e))?;
    hcr.delete_subkey(r"Directory\Background\shell\Zed")
        .map_err(|e| format!("{:?}", e))?;

    // Delete folder right-click menu
    hcu.delete_subkey(r"SOFTWARE\Classes\Directory\shell\Zed\command")
        .map_err(|e| format!("{:?}", e))?;
    hcu.delete_subkey(r"SOFTWARE\Classes\Directory\shell\Zed")
        .map_err(|e| format!("{:?}", e))?;

    // Delete file right-click menu
    hcu.delete_subkey(r"SOFTWARE\Classes\*\shell\Zed\command")
        .map_err(|e| format!("{:?}", e))?;
    hcu.delete_subkey(r"SOFTWARE\Classes\*\shell\Zed")
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}
