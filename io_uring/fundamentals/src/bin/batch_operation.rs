use io_uring::{IoUring, opcode, squeue::Flags, types};
use std::{fs::OpenOptions, io, os::fd::AsRawFd};

fn main() -> io::Result<()> {
    let mut ring = IoUring::new(8)?;

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("/tmp/uring_out.txt")?;
    let fd = types::Fd(file.as_raw_fd());

    let data = b"hello io_uring\n";

    // write -> fsync, linked so fsync only runs after write succeeds
    let write_e = opcode::Write::new(fd, data.as_ptr(), data.len() as u32)
        .offset(0)
        .build()
        .flags(Flags::IO_LINK)
        .user_data(1);

    let fsync_e = opcode::Fsync::new(fd).build().user_data(2);

    // SAFETY: `data` is a 'static slice and `file` outlives the completions.
    unsafe {
        let mut sq = ring.submission();
        sq.push(&write_e).expect("sq full");
        sq.push(&fsync_e).expect("sq full");
    } // dropping `sq` publishes the new tail to the kernel

    // Submit both in a single syscall and wait for both completions
    ring.submit_and_wait(2)?;

    for cqe in ring.completion() {
        println!("user_data={} res={}", cqe.user_data(), cqe.result());
    }
    Ok(())
}
