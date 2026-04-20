#![feature(naked_functions)]
use std::arch::asm;
use std::arch::naked_asm;

const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2;
const MAX_THREADS: usize = 4;
const F_TRAMPOLINE_OFFSET: usize = 0;
const GUARD_TRAMPOLINE_OFFSET: usize = F_TRAMPOLINE_OFFSET + std::mem::size_of::<u64>();

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
    stack: Vec<u8>,
    ctx: ThreadContext,
    state: State,
}

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    r19: u64, // 00
    r20: u64, // 08
    r21: u64, // 10
    r22: u64, // 18
    r23: u64, // 20
    r24: u64, // 28
    r25: u64, // 30
    r26: u64, // 38
    r27: u64, // 40
    r28: u64, // 48
    fp: u64,  // 50
    lr: u64,  // 58
    sp: u64,  // 60
}

impl Thread {
    fn new() -> Self {
        Thread {
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Available,
        }
    }
}

impl Runtime {
    pub fn new() -> Self {
        let base_thread = Thread {
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Running,
        };

        let mut threads = Vec::with_capacity(MAX_THREADS);
        threads.push(base_thread);
        let available_threads = (1..MAX_THREADS).map(|_| Thread::new());
        threads.extend(available_threads);

        Runtime {
            threads,
            current: 0,
        }
    }

    pub unsafe fn init(&self) {
        let r_ptr: *const Runtime = self;
        RUNTIME = r_ptr as usize;
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
            asm!(
                "bl _switch",
                in("x0") old,
                in("x1") new,
                clobber_abi("C"));
        }
        !self.threads.is_empty()
    }

    pub fn spawn(&mut self, f: fn()) {
        let available = self
            .threads
            .iter_mut()
            .find(|t| t.state == State::Available)
            .expect("no available thread.");

        let size = available.stack.len();

        // prepare stack for the trampoline
        let stack_top;
        unsafe {
            let s_end = available.stack.as_mut_ptr().add(size);
            let s_end_aligned = (s_end as usize & !15) as *mut u8;
            stack_top = s_end_aligned.offset(-16);
            std::ptr::write(stack_top.add(F_TRAMPOLINE_OFFSET).cast::<u64>(), f as u64);
            std::ptr::write(
                stack_top.add(GUARD_TRAMPOLINE_OFFSET).cast::<u64>(),
                guard as u64,
            );
        }
        available.ctx.lr = trampoline as u64;
        available.ctx.fp = 0;
        available.ctx.sp = stack_top as u64;
        available.state = State::Ready;
    }
} // We close the `impl Runtime` block here

fn guard() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_return();
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
unsafe extern "C" fn trampoline() {
    // the stack is prepared by the `Runtime::spawn`:
    // sp + 00      function_address
    // sp + 08      guard address
    naked_asm! {
        "ldr x1, [sp, {f_trampoline_offset}]",
        "ldr lr, [sp, {guard_trampoline_offset}]",
        "sub sp, sp, 0x10",  // current stack frame is not neeeded anymore
        "br x1",
        f_trampoline_offset = const F_TRAMPOLINE_OFFSET,
        guard_trampoline_offset = const GUARD_TRAMPOLINE_OFFSET,
    };
}

#[unsafe(naked)]
#[no_mangle]
unsafe extern "C" fn switch() {
    naked_asm! {
            // saving the old fiber.
            // TODO we might use `stp` instruction to store a pair of registers at once, but we don't.
            // TODO we might also use a postincrement instead of precomputed offsets.
            // But this is not an ARM tutorial, really.
            "str  x19, [x0, 0x00]",
            "str  x20, [x0, 0x08]",
            "str  x21, [x0, 0x10]",
            "str  x22, [x0, 0x18]",
            "str  x23, [x0, 0x20]",
            "str  x24, [x0, 0x28]",
            "str  x25, [x0, 0x30]",
            "str  x26, [x0, 0x38]",
            "str  x27, [x0, 0x40]",
            "str  x28, [x0, 0x48]",
            "str  fp, [x0, 0x50]",
            "str  lr, [x0, 0x58]",
            // sp cannot be stored/loaded directly -- use an intermediate register, one of the stored/loaded.
            "mov  x2, sp",
            "str  x2, [x0, 0x60]",

            // loading the new fiber
            // TODO we might use `ldp` instruction to load a pair of registers at once, but we don't.
            "ldr  x19, [x1, 0x00]",
            "ldr  x20, [x1, 0x08]",
            "ldr  x21, [x1, 0x10]",
            "ldr  x22, [x1, 0x18]",
            "ldr  x23, [x1, 0x20]",
            "ldr  x24, [x1, 0x28]",
            "ldr  x25, [x1, 0x30]",
            "ldr  x26, [x1, 0x38]",
            "ldr  x27, [x1, 0x40]",
            "ldr  x28, [x1, 0x48]",
            "ldr  fp, [x1, 0x50]",
            "ldr  lr, [x1, 0x58]",
            "ldr  x2, [x1, 0x60]",
            "mov  sp, x2",
            "ret",
    };
}

fn main() {
    let mut runtime = Runtime::new();
    // safety: we use single instance of new, use spawn and run correctly
    // TODO make runtime thread-local?
    // TODO pass runtime as an arg to spawn lambda?
    unsafe {
        runtime.init();
    }

    runtime.spawn(|| {
        println!("THREAD 1 STARTING");
        let id = 1;
        for i in 0..10 {
            println!("thread: {} counter: {}", id, i);
            yield_thread();
        }
        println!("THREAD 1 FINISHED");
    });

    runtime.spawn(|| {
        println!("THREAD 2 STARTING");
        let id = 2;
        for i in 0..15 {
            println!("thread: {} counter: {}", id, i);
            yield_thread();
        }
        println!("THREAD 2 FINISHED");
    });
    runtime.run();
}
