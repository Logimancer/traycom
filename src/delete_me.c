#include <windows.h>
#include <stdio.h>
#include <setupapi.h>
#pragma comment(lib,"setupapi.lib")

#include <DEVGUID.H>

int main(int argc, char argv[])
{

    GUID guid = GUID_DEVCLASS_PORTS;
    HDEVINFO hinfo=SetupDiGetClassDevs(
        &guid,
        NULL,
        NULL,
        //DIGCF_DEVICEINTERFACE|
        DIGCF_PRESENT|DIGCF_PROFILE);

    SP_DEVINFO_DATA devinfo;
    devinfo.cbSize=sizeof(SP_DEVINFO_DATA);

    int iDevice=0;
    while(SetupDiEnumDeviceInfo(hinfo,iDevice,&devinfo))
    {
        DWORD dwSize=0;

        BOOL ret;
        char *pszFriendlyName=NULL;

        // get the friendly name

        ret=SetupDiGetDeviceRegistryProperty(hinfo,&devinfo,SPDRP_FRIENDLYNAME,NULL,NULL
        ,0,&dwSize);
        if (ret)
        {
            pszFriendlyName = new char[dwSize];

            SetupDiGetDeviceRegistryProperty(hinfo,&devinfo,SPDRP_FRIENDLYNAME,NULL,(PBYTE)p
            szFriendlyName,dwSize,NULL);
        }

        printf("device '%s'\n", pszFriendlyName);
        if (pszFriendlyName) delete pszFriendlyName;
        iDevice++;
    }
    SetupDiDestroyDeviceInfoList(hinfo);
    return 0;
}
