use winreg::{
    enums::{HKEY_CLASSES_ROOT, HKEY_CURRENT_USER},
    RegKey,
};

#[tauri::command]
pub fn reg_right(path: &str) -> Result<(), String> {
    let hcr = RegKey::predef(HKEY_CLASSES_ROOT);
    // 文件夹内空白区域右键注册
    let (reg_name, _) = hcr
        .create_subkey("Directory\\Background\\shell\\Zed")
        .map_err(|e| format!("{:?}", e))?;
    // 注册icon图标
    reg_name
        .set_value("Icon", &path)
        .map_err(|e| format!("{:?}", e))?;
    // 注册名称
    reg_name
        .set_value("", &"Open in Zed")
        .map_err(|e| format!("{:?}", e))?;
    // command reg
    let (reg_comm, _) = hcr
        .create_subkey("Directory\\Background\\shell\\Zed\\command")
        .map_err(|e| format!("{:?}", e))?;
    reg_comm
        .set_value("", &format!("{} \"%V\"", path))
        .map_err(|e| format!("{:?}", e))?;
    // 注册文件右键
    let hcu = RegKey::predef(HKEY_CURRENT_USER);
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
    // 删除文件夹右键
    let hcr = RegKey::predef(HKEY_CLASSES_ROOT);
    hcr.delete_subkey(r"Directory\Background\shell\Zed\command")
        .map_err(|e| format!("{:?}", e))?;
    hcr.delete_subkey(r"Directory\Background\shell\Zed")
        .map_err(|e| format!("{:?}", e))?;
    // 删除文件右键
    let hcu = RegKey::predef(HKEY_CURRENT_USER);
    hcu.delete_subkey(r"SOFTWARE\Classes\*\shell\Zed\command")
        .map_err(|e| format!("{:?}", e))?;
    hcu.delete_subkey(r"SOFTWARE\Classes\*\shell\Zed")
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}
