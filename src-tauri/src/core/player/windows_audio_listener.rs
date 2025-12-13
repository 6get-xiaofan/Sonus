use output_stream_manager::{DeviceInfo, OutputStreamManager};
use anyhow::{anyhow, Result};
use com_rs::com_interface;
use parking_lot::Mutex;
use std::ptr::null_mut;
use std::sync::Arc;
use winapi::{
    shared::guiddef::GUID,
    shared::minwindef::{BOOL, DWORD, HRESULT, LPVOID, ULONG},
    shared::winerror::{E_NOINTERFACE, S_OK},
    um::{
        mmdeviceapi::{
            IMMDevice, IMMDeviceEnumerator, IMMNotificationClient, EDataFlow, ERole,
            MMDeviceEnumerator,
        },
        ole32::{CoInitializeEx, CoUninitialize, CLSCTX_ALL},
        propidlbase::PROPVARIANT,
        unknwnbase::IUnknown,
        winbase::COINIT_APARTMENTTHREADED,
    },
};

#[com_interface("7991EEC9-7E89-4D85-8390-6C703CEC60C0")]
pub trait IMMNotificationClient: IUnknown {
    unsafe fn OnDeviceStateChanged(
        &self,
        pwstrDeviceId: *const u16,
        dwNewState: DWORD,
    ) -> HRESULT;
}