use crate::consts::PIPESIZE;

struct Pipe {
    data: [u8; PIPESIZE],
    nread: usize,
    nwrite: usize,
    readopen: usize,
    writeopen: usize,
}
