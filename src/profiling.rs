use std::{fs::File, io::{BufWriter, Write}, time::Instant};

use lazy_static::lazy_static;

struct ProfileSession {
    out: BufWriter<File>
}

unsafe fn very_bad_function<T>(reference: &T) -> &mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}

impl ProfileSession {
    fn new() -> Self {
        let mut out = BufWriter::new(File::open("profile.json").unwrap());

        out.write_all(r#"{"otherData": {}, "traceEvents": ["#.as_bytes()).unwrap();

        ProfileSession {
            out
        }
    }
}

lazy_static! {
    static ref PROFILE_SESSION: ProfileSession = ProfileSession::new();
}

struct ProfileTimer {
    name: &'static str,
    start: Instant,
    stopped: bool
}

impl ProfileTimer {
    pub fn new(name: &'static str) -> Self {
        ProfileTimer {
            name,
            start: Instant::now(),
            stopped: false
        }
    }

    pub fn stop(&mut self) {
        if self.stopped {
            return;
        }

        self.stopped = true;

        let end = Instant::now();
    }
}