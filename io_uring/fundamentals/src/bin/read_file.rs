use io_uring::{IoUring, opcode, types};
use std::{
    fs::File,
    io,
    os::{fd::AsRawFd, fs::AsRawFd},
};

fn main() -> io::Result<()> {
    // create a ring with 8 SQ entries (CQ defaults to 2x that)
    let mut ring = IoUring::new(8)?;

    let file = File::open("/etc/hostname")?;
    let mut buf = vec![0u8; 1024];

    // build a Read SQE: read up to buf.len() bytes at offset 0
    let read_e = opcode::Read::new(
        types::Fd(file.as_raw_fd()),
        buf.as_mut_ptr(),
        buf.len() as u32,
    )
    .offset(0)
    .build()
    .user_data(0x42); // used to match CQE later

    unsafe {
        ring.submission()
            .push(&read_e)
            .expect("submission queue is full");
    }

    // 1 syscall: submit 1 SQE and wait for at least 1 completion
    ring.submit_and_wait(1)?;

    // reap the completion
    let cqe = ring.completion().next().expect("completion queue is empty");
    assert_eq!(cqe.user_data(), 0x42);

    let res = cqe.result();
    if res < 0 {
        // negative result is -errno
        return Err(io::Error::from_raw_os_error(-res));
    }

    let n = res as usize;
    println!("read {n} bytes: {}:", String::from_utf8_lossy(&buf[..n]));
    Ok(())
}
