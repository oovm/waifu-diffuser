use super::*;

impl WaifuDiffuserServer {
    pub fn singleton() -> &'static WaifuDiffuserServer {
        // Create an uninitialized static
        static mut SINGLETON: MaybeUninit<WaifuDiffuserServer> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();
        unsafe {
            ONCE.call_once(|| {
                // Make it
                let singleton = WaifuDiffuserServer { inner: Mutex::new(0) };
                // Store it to the static var, i.e. initialize it
                SINGLETON.write(singleton);
            });
            // Now we give out a shared reference to the data, which is safe to use concurrently.
            SINGLETON.assume_init_ref()
        }
    }
}
