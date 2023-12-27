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
async fn deviceport(serial_device: IAsyncOperation<SerialDevice>) -> SerialDevice {
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
    let serial_device_async = SerialDevice::FromIdAsync(&deviceid);
    let serial_device = task::block_on(deviceport(serial_device_async.unwrap()));
    let serial_return = serial_device.PortName().unwrap();
    serial_return 
}

fn main() {
    let serial_devices_information_collection: DeviceInformationCollection = get_serial_devices();
    //let port_name: windows::core::HSTRING = serial_devices.1; 
    //Print serial device names
    for serial_device in serial_devices_information_collection {
        println!("{} {}", serial_device_comm_number(serial_device.Id().unwrap()), serial_device.Name().unwrap());
//        for serial_device_properties in serial_device.Properties() {
//            println!("{}", serial_device_properties.Lookup("Name").unwrap() );
        //}
    }

}   

/* use windows::Win32::{Devices::Communication::GetCommPorts,
                    Foundation::{ERROR_SUCCESS, ERROR_MORE_DATA, ERROR_FILE_NOT_FOUND}};


                    fn get_commports() -> Reslut< {
    unsafe {
        //Setup Pointers
        let lpportnumbers:&mut [u32] = &mut [0;16];
        
        //This a derived raw pointer to an array
        let mut puportnumbersfound: [u32;8] = [0;8];
        let puportnumbersfound_raw: *mut u32 = &mut puportnumbersfound as *mut u32;

        let get_comm_ports_return: u32 = GetCommPorts(lpportnumbers, puportnumbersfound_raw);

        //Handle Errors
        if get_comm_ports_return == ERROR_SUCCESS.0 {
            println!("The Call Succeeded.")
        } else if get_comm_ports_return == ERROR_MORE_DATA.0 {
            println!("The lpportnumbers array was too smol.")
        } else if get_comm_ports_return == ERROR_FILE_NOT_FOUND.0 {
            println!("There are no commports available.")
        }

        //Print number of ports
        for i in 0..puportnumbersfound.len() {
            if puportnumbersfound[i] != 0 {
                println!("{} port(s) found.", puportnumbersfound[i])
            }
        }

        //Print port results
        for i in 0..lpportnumbers.len() {
            if lpportnumbers[i] != 0 {
                print!("COM{} ", lpportnumbers[i]);
                
            }
        }
    }
} */