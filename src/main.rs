#![windows_subsystem = "windows"] //keeps a console from opening
use windows::{Devices::{
    SerialCommunication::SerialDevice, 
    Enumeration::{
        DeviceInformation, 
        DeviceInformationCollection
        }
    }, 
    Foundation::IAsyncOperation, 
    core::{Error, HSTRING}
};
use async_std::task;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, IsMenuItem}, 
    TrayIconBuilder};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

//Async wrapper for getting device info
pub async fn serial_ports_device_info(
    deviceinformation: IAsyncOperation<DeviceInformationCollection>) 
    -> DeviceInformationCollection {
        let response: Result<DeviceInformationCollection, Error> = 
            deviceinformation.await;
        response.unwrap()
}

//Async wrapper for getting port number
pub async fn deviceport(serial_device: IAsyncOperation<SerialDevice>) 
    -> SerialDevice {
    let response: Result<SerialDevice, Error> = 
        serial_device.await;
    response.unwrap()
}

//Create Device information from SerialDevices
fn get_serial_devices() -> DeviceInformationCollection  {
    let deviceid: windows::core::HSTRING = 
        SerialDevice::GetDeviceSelector().unwrap();
    //Get device information for name
    let deviceinformation: 
        Result<IAsyncOperation<DeviceInformationCollection>, Error> = 
        DeviceInformation::FindAllAsyncAqsFilter(&deviceid);
    let dev_info_collection: DeviceInformationCollection = 
        task::block_on(serial_ports_device_info(
            deviceinformation.unwrap()
        )
    );
    dev_info_collection
}

fn refresh_serial_ports() -> Vec<MenuItem> {
    let mut menu_items_ports: Vec<MenuItem> = Vec::new();
    let serial_devices_information_collection: 
        DeviceInformationCollection = get_serial_devices();
    let serial_devices: Vec<(HSTRING, HSTRING)> = 
        enumerate_serial_devices(
            serial_devices_information_collection);
    for serial_device in serial_devices {
        //iterate through each serial device and add to menu
        let current_menu_item = 
            serial_device.0.to_string() + " " + &serial_device.1.to_string();
        menu_items_ports.push(
            MenuItem::new(current_menu_item, 
            false,
            Option::None));
    }
    menu_items_ports
}

fn enumerate_serial_devices(
    serial_device_information_collection: DeviceInformationCollection) -> 
    Vec<(HSTRING, HSTRING)> {    
    let mut serial_devices:Vec<(HSTRING, HSTRING)> = Vec::new(); 
    for serial_device in serial_device_information_collection {
        let serial_device_id = serial_device_comm_number(
            serial_device.Id().unwrap());
        let serial_device_name = serial_device.Name().unwrap();
        serial_devices.push((serial_device_id, serial_device_name));
    }
    serial_devices
}

fn serial_device_comm_number(deviceid: HSTRING) -> windows::core::HSTRING {
    let serial_device_async: Result<IAsyncOperation<SerialDevice>, Error> = 
        SerialDevice::FromIdAsync(&deviceid);
    let serial_device: SerialDevice = 
        task::block_on(
            deviceport(serial_device_async.unwrap()
        )
    );
    let serial_return: HSTRING = serial_device.PortName().unwrap();
    serial_return 
}
    
fn build_menu_ports(menu: &Menu, menu_items_ports: Vec<MenuItem>) -> Menu {
    //Add Ports to menu
    for port in menu_items_ports {
        let _ = menu.prepend(&port);
    }
    menu.clone()
}

fn remove_current_menu_ports(menu: &Menu) {
    let i = 1;
    for item in menu.items() {
        if item.as_menuitem().is_none() { break; }
        println!("{}", i);
        let _ = menu.remove(item.as_menuitem().unwrap());
    }
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba: Vec<u8> = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn main() {    
    //setup tray icon
    let path: &str = "icon\\icon.ico";
    let icon: tray_icon::Icon = load_icon(std::path::Path::new(path));
    
    //build event loop for tray (req'd)
    let event_loop: winit::event_loop::EventLoop<()> = 
        EventLoopBuilder::new().build().unwrap();
    
    //Setup tray menu and tray menu items
    let menu: Menu = Menu::new();
    //Seperator definition
    let menu_item_seperator = PredefinedMenuItem::separator();
    //Refresh definition
    let menu_item_refresh = MenuItem::new(
        "Refresh", true, Option::None);
    //Quit definition
    let menu_item_quit: MenuItem = MenuItem::new(
            "Quit", true, Option::None);
    //Add control items after ports to menu
    let persistant_menu_items: [&dyn IsMenuItem; 4] = [&menu_item_seperator, 
                                                       &menu_item_refresh,
                                                       &menu_item_seperator,
                                                       &menu_item_quit];   
        
    //build initial serial port portion of menu
    let menu: Menu = build_menu_ports(&menu, refresh_serial_ports());
    //add persistent control portion of menu
    let _ = menu.append_items(&persistant_menu_items);

    //display tray icon
    let _tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(menu.clone()))
            .with_tooltip("Serial Port Status") 
            .with_icon(icon)
            .build()
            .unwrap(),
    );

    //event loop logic
    let menu_channel = MenuEvent::receiver();
    let _ = event_loop.run(move |_event: winit::event::Event<()>, 
        event_loop: &winit::event_loop::EventLoopWindowTarget<()>| {
        event_loop.set_control_flow(ControlFlow::Poll);
        if let Ok(event) = menu_channel.try_recv() {
            println!("{event:?}");
            
            //Refresh action
            if event.id.0 == menu_item_refresh.id().0 {
                remove_current_menu_ports(&menu);
                build_menu_ports(&menu, refresh_serial_ports());
            }
            //Quit action
            if event.id.0 == menu_item_quit.id().0 {
               event_loop.exit(); 
            }
        }
    });
} 