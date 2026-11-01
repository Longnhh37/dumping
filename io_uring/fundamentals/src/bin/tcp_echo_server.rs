use io_uring::{IoUring, cqueue, opcode, squeue, types};
use slab::Slab;
use std::{
    io,
    net::TcpListener,
    os::fd::{AsRawFd, RawFd},
};

#[derive(Debug)]
enum Op {
    Accept,
    Recv {
        fd: RawFd,
        buf_idx: usize,
    },
    Send {
        fd: RawFd,
        buf_idx: usize,
        sent: usize,
        len: usize,
    },
}

const BUF_SIZE: usize = 2048;

fn push(ring: &mut IoUring, sqe: squeue::Entry) {
    unsafe {
        while ring.submission().push(&sqe).is_err() {
            ring.submit().expect("submit failed");
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let listener_fd = listener.as_raw_fd();

    let mut ring = IoUring::new(256)?;
    let mut ops: Slab<Op> = Slab::with_capacity(256);
    let mut bufs: Slab<Box<[u8; BUF_SIZE]>> = Slab::with_capacity(256);

    let key = ops.insert(Op::Accept);
    push(
        &mut ring,
        opcode::Accept::new(
            types::Fd(listener_fd),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
        .build()
        .user_data(key as u64),
    );

    loop {
        ring.submit_and_wait(1)?;
        let cqes: Vec<cqueue::Entry> = ring.completion().collect();

        for cqe in cqes {
            let key = cqe.user_data() as usize;
            let res = cqe.result();
            let op = ops.remove(key);

            match op {
                Op::Accept => {
                    let k = ops.insert(Op::Accept);
                    push(
                        &mut ring,
                        opcode::Accept::new(
                            types::Fd(listener_fd),
                            std::ptr::null_mut(),
                            std::ptr::null_mut(),
                        )
                        .build()
                        .user_data(k as u64),
                    );

                    if res < 0 {
                        eprintln!("accept error: {}", io::Error::from_raw_os_error(-res));
                        continue;
                    }
                    let fd = res as RawFd;
                    let buf_idx = bufs.insert(Box::new([0u8; BUF_SIZE]));
                    let ptr = bufs[buf_idx].as_mut_ptr();
                    let k = ops.insert(Op::Recv { fd, buf_idx });
                    push(
                        &mut ring,
                        opcode::Recv::new(types::Fd(fd), ptr, BUF_SIZE as u32)
                            .build()
                            .user_data(k as u64),
                    );
                }

                Op::Recv { fd, buf_idx } => {
                    if res <= 0 {
                        bufs.remove(buf_idx);
                        unsafe { libc::close(fd) };
                        continue;
                    }
                    let len = res as usize;
                    let ptr = bufs[buf_idx].as_ptr();
                    let k = ops.insert(Op::Send {
                        fd,
                        buf_idx,
                        sent: 0,
                        len,
                    });
                    push(
                        &mut ring,
                        opcode::Send::new(types::Fd(fd), ptr, len as u32)
                            .build()
                            .user_data(k as u64),
                    );
                }

                Op::Send {
                    fd,
                    buf_idx,
                    sent,
                    len,
                } => {
                    if res < 0 {
                        bufs.remove(buf_idx);
                        unsafe { libc::close(fd) };
                        continue;
                    }
                    let sent = sent + res as usize;
                    if sent < len {
                        let ptr = unsafe { bufs[buf_idx].as_ptr().add(sent) };
                        let k = ops.insert(Op::Send {
                            fd,
                            buf_idx,
                            sent,
                            len,
                        });
                        push(
                            &mut ring,
                            opcode::Send::new(types::Fd(fd), ptr, (len - sent) as u32)
                                .build()
                                .user_data(k as u64),
                        );
                    } else {
                        let ptr = bufs[buf_idx].as_mut_ptr();
                        let k = ops.insert(Op::Recv { fd, buf_idx });
                        push(
                            &mut ring,
                            opcode::Recv::new(types::Fd(fd), ptr, BUF_SIZE as u32)
                                .build()
                                .user_data(k as u64),
                        );
                    }
                }
            }
        }
    }
}
