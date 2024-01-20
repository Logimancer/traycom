# TrayCom

## A system tray application to show which COM ports are currently available

The target user for this program is anyone who needs fast access to com port information, e.g. controls engineers and technicians that are tired of opening windows device manager every time they plug in a new USB to RS-232/485 converter.

TrayCom is written in 100% Rust and has been tested on windows 10 and windows 11.


Please check the [release section](https://github.com/Logimancer/traycom/releases) for the latest installer.


**This is an alpha release!** 

Please report any bugs or feature requests on the github issues page.


I hope you find TrayCom useful :)

# Screenshots:

TrayCom in system tray (starts up automatically on login):

![system tray](https://github.com/Logimancer/traycom/assets/151105595/cf868700-e65a-4931-805e-7bd3284e3ca3)

Right-clicking on the TrayCom icon will show current connected devices.

This is what TrayCom shows with no COM ports present on the computer. Clicking the "Refresh" menu item will check for new COM ports:

![refresh](https://github.com/Logimancer/traycom/assets/151105595/0d5c8006-d178-434b-b759-44f0a29b2ce5)

After clicking the "Refresh" menu item, TrayCom now shows the two new USB to RS232 devices that have just been connected, along with their COM port numbers:

![ports](https://github.com/Logimancer/traycom/assets/151105595/50891204-906a-429a-b49e-8e07706bd888)

*Note: there may be COM ports on the list that are not external to the computer.* 

An example of this is the [Intel Active Management Technology](https://en.wikipedia.org/wiki/Intel_Active_Management_Technology) ["SOL" device](https://en.wikipedia.org/wiki/Serial_over_LAN), which usually reports back as COM3.

All USB devices that have been tested so far show "USB" in their listing in the TrayCom menu.

This can be useful in determining which COM ports are able to be used for serial connections with external equipment (traditional USB to Serial devices, for instance).
