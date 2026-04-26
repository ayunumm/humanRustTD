use nrf52840_pac as pac;

pub fn disable_approtect() {
    unsafe {
        let uicr = &*pac::UICR::ptr();
        let nvmc = &*pac::NVMC::ptr();

        nvmc.config.write(|w| w.wen().wen());
        while nvmc.ready.read().ready().is_busy() {}

        uicr.approtect.write(|w| w.bits(0xFFFFFFFF));
        while nvmc.ready.read().ready().is_busy() {}

        nvmc.config.write(|w| w.wen().ren());
        while nvmc.ready.read().ready().is_busy() {}     
    }
}
