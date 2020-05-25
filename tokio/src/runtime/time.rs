//! Abstracts out the APIs necessary to `Runtime` for integrating the time
//! driver. When the `time` feature flag is **not** enabled. These APIs are
//! shells. This isolates the complexity of dealing with conditional
//! compilation.

pub(crate) use variant::*;

#[cfg(feature = "time")]
mod variant {
    use crate::park::Either;
    use crate::runtime::io;
    use crate::time::{self, driver};

    pub(crate) type Clock = time::Clock;
    pub(crate) type Driver = Either<driver::Driver<io::Driver>, io::Driver>;
    pub(crate) type Handle = Option<driver::Handle>;

    pub(crate) fn create_clock() -> Clock {
        Clock::new()
    }

    #[cfg(all(feature = "test-util", tokio_unstable))]
    pub(crate) fn create_test_clock() -> Clock {
        Clock::new_frozen()
    }

    /// Create a new timer driver / handle pair
    pub(crate) fn create_driver<P>(
        enable: bool,
        io_driver: P,
        clock: Clock,
    ) -> (Either<driver::Driver<P>, P>, Handle)
    where
        P: crate::park::Park,
    {
        if enable {
            let driver = driver::Driver::new(io_driver, clock);
            let handle = driver.handle();

            (Either::A(driver), Some(handle))
        } else {
            (Either::B(io_driver), None)
        }
    }
}

#[cfg(not(feature = "time"))]
mod variant {
    use crate::runtime::io;

    pub(crate) type Clock = ();
    pub(crate) type Driver = io::Driver;
    pub(crate) type Handle = ();

    pub(crate) fn create_clock() -> Clock {
        ()
    }

    #[cfg(all(feature = "test-util", tokio_unstable))]
    pub(crate) fn create_test_clock() -> Clock {
        ()
    }

    /// Create a new timer driver / handle pair
    pub(crate) fn create_driver<P>(_enable: bool, io_driver: P, _clock: Clock) -> (P, Handle)
    where
        P: crate::park::Park,
    {
        (io_driver, ())
    }
}
