use windows::{Devices::{
    SerialCommunication::SerialDevice, 
    Enumeration::{
        DeviceInformation, 
        DeviceInformationCollection
        }
    }, 
    Foundation::IAsyncOperation, 
    core::Error
};
use async_std::task;

//Async wrapper for getting device info
pub async fn serial_ports_device_info(
    deviceinformation: IAsyncOperation<DeviceInformationCollection>) 
    -> DeviceInformationCollection {
        let response: Result<DeviceInformationCollection, Error> = 
            deviceinformation.await;
        return response.unwrap();
}

fn get_serial_devices() -> DeviceInformationCollection {
    let deviceid: Result<windows::core::HSTRING, Error> = 
        SerialDevice::GetDeviceSelector();
    let deviceinformation: Result<IAsyncOperation<DeviceInformationCollection>, Error> = 
        DeviceInformation::FindAllAsyncAqsFilter(&deviceid.unwrap());
    let dev_info_collection: DeviceInformationCollection = 
        task::block_on(serial_ports_device_info(
            deviceinformation.unwrap()
        )
    );
    return dev_info_collection;
}

fn main() {
    let serial_devices: DeviceInformationCollection = get_serial_devices();
    
    //Print serial device names
    for serial_device in serial_devices {
        println!("{}", serial_device.Name().unwrap());
        //println!("{}", serial_device.Id().);
        //for serial_device_properties in serial_device.Properties() {
        //    println!("{}", serial_device_properties.Lookup("Name").unwrap() );
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