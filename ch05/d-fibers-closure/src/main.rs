/// FIX #31:
/// Inline assembly blocks inside naked functions now need to use
/// the `naked_asm` macro instead of the good old `asm` macro.
/// The `noreturn` option is implicitly set by the `naked_asm`
/// macro so there is no need to set that.
///
/// See: https://github.com/PacktPublishing/Asynchronous-Programming-in-Rust/issues/31
/// for more information.
use std::{arch::{asm, naked_asm}};

const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2;
const MAX_THREADS: usize = 4;
static mut RUNTIME: usize = 0;

pub struct Runtime {
    threads: Vec<Thread>,
    current: usize,
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Available,
    Running,
    Ready,
}

struct Thread {
    id: usize, // changed
    stack: Vec<u8>,
    ctx: ThreadContext,
    state: State,
    task: Option<Box<dyn FnOnce()>>, // changed
}

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
    thread_ptr: u64,
}

impl Thread {
    fn new(id: usize) -> Self {
        Thread {
            id, // changed
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Available,
            task: None,
        }
    }
}

impl Runtime {
    pub fn new() -> Self {
        let base_thread = Thread {
            id: 0, //changed
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Running,
            task: None,
        };

        let mut threads = vec![base_thread];
        threads[0].ctx.thread_ptr = &threads[0] as *const Thread as u64;
        // changed (assign "id")
        let mut available_threads: Vec<Thread> = (1..MAX_THREADS).map(|i| Thread::new(i)).collect();
        threads.append(&mut available_threads);

        Runtime {
            threads,
            current: 0,
        }
    }

    pub fn init(&self) {
        unsafe {
            let r_ptr: *const Runtime = self;
            RUNTIME = r_ptr as usize;
        }
    }

    pub fn run(&mut self) -> ! {
        while self.t_yield() {}
        std::process::exit(0);
    }

    fn t_return(&mut self) {
        if self.current != 0 {
            self.threads[self.current].state = State::Available;
            self.t_yield();
        }
    }

    #[inline(never)]
    fn t_yield(&mut self) -> bool {
        let mut pos = self.current;
        while self.threads[pos].state != State::Ready {
            pos += 1;
            if pos == self.threads.len() {
                pos = 0;
            }
            if pos == self.current {
                return false;
            }
        }

        if self.threads[self.current].state != State::Available {
            self.threads[self.current].state = State::Ready;
        }

        self.threads[pos].state = State::Running;
        let old_pos = self.current;
        self.current = pos;

        unsafe {
            let old: *mut ThreadContext = &mut self.threads[old_pos].ctx;
            let new: *const ThreadContext = &self.threads[pos].ctx;
            asm!("call switch", in("rdi") old, in("rsi") new, clobber_abi("C"));
        }
        self.threads.len() > 0
    }

    pub fn spawn<F: FnOnce() + 'static>(f: F) { // changed
        unsafe {
            let rt_ptr = RUNTIME as *mut Runtime;
            let available = (*rt_ptr)
                .threads
                .iter_mut()
                .find(|t| t.state == State::Available)
                .expect("no available thread.");

            let size = available.stack.len();
            let s_ptr = available.stack.as_mut_ptr().offset(size as isize);
            let s_ptr = (s_ptr as usize & !15) as *mut u8;
            available.task = Some(Box::new(f));
            available.ctx.thread_ptr = available as *const Thread as u64;
            std::ptr::write(s_ptr.offset(-16) as *mut u64, guard as u64);
            std::ptr::write(s_ptr.offset(-24) as *mut u64, skip as u64);
            std::ptr::write(s_ptr.offset(-32) as *mut u64, call as u64); // changed
            available.ctx.rsp = s_ptr.offset(-32) as u64;
            available.state = State::Ready;
        }
    }
}

// This function is new
fn call(thread: u64) {
    let thread = unsafe { &mut *(thread as *mut Thread) };
    if let Some(f) = thread.task.take() {
        f();
    }
}

#[unsafe(naked)]
unsafe extern "C" fn skip() {
    naked_asm!("ret")
}

// this function is changed
fn guard() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        let rt = &mut *rt_ptr;
        println!("THREAD {} FINISHED.", rt.threads[rt.current].id);
        rt.t_return();
    };
}

pub fn yield_thread() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_yield();
    };
}
#[unsafe(naked)]
#[no_mangle]
#[cfg_attr(target_os = "macos", export_name = "\x01switch")]
unsafe extern "C" fn switch() {
    naked_asm!(
        "mov 0x00[rdi], rsp",
        "mov 0x08[rdi], r15",
        "mov 0x10[rdi], r14",
        "mov 0x18[rdi], r13",
        "mov 0x20[rdi], r12",
        "mov 0x28[rdi], rbx",
        "mov 0x30[rdi], rbp",
        "mov rsp, 0x00[rsi]",
        "mov r15, 0x08[rsi]",
        "mov r14, 0x10[rsi]",
        "mov r13, 0x18[rsi]",
        "mov r12, 0x20[rsi]",
        "mov rbx, 0x28[rsi]",
        "mov rbp, 0x30[rsi]",
        "mov rdi, 0x38[rsi]",
        "ret"
    );
}

// The main function has also changed
#[cfg(not(windows))]
pub fn main() {
    let mut runtime = Runtime::new();
    runtime.init();
    Runtime::spawn(|| {
        println!("I haven't implemented a timer in this example.");
        yield_thread();
        println!("Finally, notice how the tasks are executed concurrently.");
    });
    Runtime::spawn(|| {
        println!("But we can still nest tasks...");
        Runtime::spawn(|| {
            println!("...like this!");
        })
    });
    runtime.run();
}
#[cfg(windows)]
fn main() {}
