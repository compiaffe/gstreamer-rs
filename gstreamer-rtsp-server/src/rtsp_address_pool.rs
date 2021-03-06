use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;
use RTSPAddress;
use RTSPAddressPool;
use RTSPAddressPoolResult;

pub trait RTSPAddressPoolExtManual: 'static {
    fn reserve_address(
        &self,
        ip_address: &str,
        port: u32,
        n_ports: u32,
        ttl: u32,
    ) -> Result<RTSPAddress, RTSPAddressPoolResult>;
}

impl<O: IsA<RTSPAddressPool>> RTSPAddressPoolExtManual for O {
    fn reserve_address(
        &self,
        ip_address: &str,
        port: u32,
        n_ports: u32,
        ttl: u32,
    ) -> Result<RTSPAddress, RTSPAddressPoolResult> {
        unsafe {
            let mut address = ptr::null_mut();
            let ret = from_glib(ffi::gst_rtsp_address_pool_reserve_address(
                self.as_ref().to_glib_none().0,
                ip_address.to_glib_none().0,
                port,
                n_ports,
                ttl,
                &mut address,
            ));
            match ret {
                RTSPAddressPoolResult::Ok => Ok(from_glib_full(address)),
                _ => Err(ret),
            }
        }
    }
}
