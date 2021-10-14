
pub mod interface {
    pub trait DeviceDriver {
        // return a compatibility string for identifying the driver.

        fn compatible(&self) -> &'static str;


        /// called by the kernel to bring up the device.

        unsafe fn init(&self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    pub trait DriverManager {
        fn all_device_drivers(&self) -> &[&'static (dyn DeviceDriver + Sync)];

        fn post_device_driver_init(&self);
    }
}

