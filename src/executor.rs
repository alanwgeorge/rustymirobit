use cortex_m::asm;
use heapless::mpmc::Q4;
use rtt_target::rprintln;
use crate::future::OurFuture;

pub fn wake_task(task_id: usize) {
    rprintln!("Wake task {}", task_id);
    if TASK_ID_READY.enqueue(task_id).is_err() {
        panic!("Task ID queue full; cannot add task {}", task_id);
    }
}

static TASK_ID_READY: Q4<usize> = Q4::new();

pub fn run_tasks(tasks: &mut [&mut dyn OurFuture<()>]) -> ! {
    for task_id in 0..tasks.len() {
        TASK_ID_READY.enqueue(task_id).ok();
    }

    loop {
        while let Some(task_id) = TASK_ID_READY.dequeue() {
            if task_id >= tasks.len() {
                rprintln!("Task ID out of bounds");
                continue;
            }
            rprintln!("[executor]: task id = {:?}", task_id);
            tasks[task_id].poll(task_id);
        }
        rprintln!("[executor]: tasks done, going to sleep....");
        asm::wfi();
    }
}