use insomnia::{InhibitionManager, Lock, LockType};
use tracing::{trace, warn};

pub fn inhibit_sleep() -> Option<Box<dyn Lock>> {
    match insomnia::manager() {
        Ok(manager) => {
            match manager.lock(
                LockType::AutomaticSuspend | LockType::Screen,
                env!("CARGO_PKG_NAME"),
                "Keep mission critical information and tools available",
            ) {
                Ok(sleep_lock) => {
                    trace!("Inhibiting sleep");
                    Some(Box::new(sleep_lock))
                }
                Err(error) => {
                    warn!(%error, "Failed inhibit sleep");
                    None
                }
            }
        }
        Err(error) => {
            warn!(%error, "Failed connect to sleep manager");

            None
        }
    }
}
