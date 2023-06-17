use registry::{Data, Error, Hive, Security};
use std::{
    ffi::{c_void, OsString},
    io::stdin,
    os::raw::{c_char, c_int},
    process::Command,
    sync::mpsc::Receiver,
    time::Duration,
};
use windows_service::{
    service::{ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceType},
    service_manager::{ServiceManager, ServiceManagerAccess},
};

fn service_main() -> windows_service::Result<()> {
    let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE | ServiceManagerAccess::all();
    let service_manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_binary_path = ::std::env::current_exe()
        .unwrap()
        .with_file_name("rusty-windows.exe");

    let service_info = ServiceInfo {
        name: OsString::from("rusty-windows2"),
        display_name: OsString::from("Rusty Windows 2"),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None,
        account_password: None,
    };
    let service = service_manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)?;
    service.set_description("Fix the stupid windows")?;
    Ok(())
}

fn main() -> Result<(), Error> {
    if let Err(e) = service_main() {
        println!("{}", e)
    }
    // let regkey = Hive::CurrentUser.open(
    //     r"Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced",
    //     Security::AllAccess,
    // )?;
    // regkey.set_value("HideFileExt", &Data::U32(0))?;
    // let val = regkey.value("HideFileExt")?;
    // println!("{}", val);
    // let mut buffer = String::new();
    // let stdin = stdin();
    // stdin.read_line(&mut buffer).unwrap();
    // println!("{}", buffer);
    return Ok(());
}

struct Regedit {
    regkey_hide_file_ext: String,
}
