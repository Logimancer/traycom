//#![windows_subsystem = "windows"] //keeps a console from opening
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
    menu::{Menu, MenuEvent, MenuItem}, TrayIconBuilder, TrayIconEvent};
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
    let deviceinformation: Result<IAsyncOperation<DeviceInformationCollection>, Error> = 
        DeviceInformation::FindAllAsyncAqsFilter(&deviceid);
    let dev_info_collection: DeviceInformationCollection = 
        task::block_on(serial_ports_device_info(
            deviceinformation.unwrap()
        )
    );
    dev_info_collection
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

fn enumerate_serial_devices(serial_device_information_collection: DeviceInformationCollection) -> 
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

fn main() {    
    //TODO: Functionalize this so it can be called each time we need to update the port information
    let serial_devices_information_collection: DeviceInformationCollection = 
    get_serial_devices();
    //enumerate serial devices
    let serial_devices = 
        enumerate_serial_devices(serial_devices_information_collection);
    for serial_device in serial_devices {
        println!("{} {}", serial_device.0, serial_device.1)
    }
    let path: &str = "icon\\icon.ico";
    let icon: tray_icon::Icon = load_icon(std::path::Path::new(path));
    let event_loop: winit::event_loop::EventLoop<()> = EventLoopBuilder::new().build().unwrap();
    let menu: Menu = Menu::new();    
    //TODO: Show comports in menu, iterating through list made above:
    let port_menu_item: MenuItem = MenuItem::new("Insert COM Port details here", 
        false, Option::None);
    let menu_item_quit: MenuItem = MenuItem::new(
            "Quit", true, Option::None);
    let _ = menu.append_items(&[&port_menu_item, &menu_item_quit]);
    let _tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("{}", /* amount of ports */)
            .with_icon(icon)
            .build()
            .unwrap(),
    );
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();
    let _ = event_loop.run(move |_event: winit::event::Event<()>, 
        event_loop: &winit::event_loop::EventLoopWindowTarget<()>| {
        event_loop.set_control_flow(ControlFlow::Poll);
        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }
        if let Ok(event) = menu_channel.try_recv() {
            println!("{event:?}");
            if event.id.0 == "1002" {
               event_loop.exit(); 
            }
        }
    });
} 