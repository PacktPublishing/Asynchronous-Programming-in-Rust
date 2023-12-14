use std::sync::{Mutex, Condvar};

#[derive(Default)]
pub struct Parker(Mutex<bool>, Condvar);

impl Parker {
    pub fn park(&self) {

        // We aquire a lock to the Mutex which protects our flag indicating if we
        // should resume execution or not.
        let mut resumable = self.0.lock().unwrap();

            // We put this in a loop since there is a chance we'll get woken, but
            // our flag hasn't changed. If that happens, we simply go back to sleep.
            while !*resumable {

                // We sleep until someone notifies us
                resumable = self.1.wait(resumable).unwrap();
            }

        // We immidiately set the condition to false, so that next time we call `park` we'll
        // go right to sleep.
        *resumable = false;
    }

    pub fn unpark(&self) {
        // We simply acquire a lock to our flag and sets the condition to `runnable` when we
        // get it.
        *self.0.lock().unwrap() = true;

        // We notify our `Condvar` so it wakes up and resumes.
        self.1.notify_one();
    }
}


#[test]
fn parker_works() {
    use std::{thread, time, sync::{Arc, atomic::{AtomicBool, Ordering}}};

    let flag = Arc::new(AtomicBool::new(false));
    let parker = Arc::new(Parker::default());

    let flag_clone = flag.clone();
    let parker_clone = parker.clone();

    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(200));
        flag_clone.store(true, Ordering::SeqCst);
        parker_clone.unpark();
    });
    assert!(!flag.load(Ordering::SeqCst), "Flag should be false at this point!");
    parker.park();
    assert!(flag.load(Ordering::SeqCst), "Flag should be true at this point!");
}