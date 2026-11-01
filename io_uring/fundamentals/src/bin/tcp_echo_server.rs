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
