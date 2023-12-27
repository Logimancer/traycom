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

fn main() {
    let serial_devices_information_collection: DeviceInformationCollection = 
    get_serial_devices();
    for serial_device in serial_devices_information_collection {
        println!("{} {}", serial_device_comm_number(serial_device.Id().unwrap()), serial_device.Name().unwrap());
    }
}  