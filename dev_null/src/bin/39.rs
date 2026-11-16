use std::time::{SystemTime, UNIX_EPOCH};

const EPOCH_MS: u64 = 1_735_689_600_000;
const MACHINE_BITS: u64 = 10;
const SEQ_BITS: u64 = 12;
const MAX_SEQ: u64 = (1 << SEQ_BITS) - 1;

struct Snowflake {
    machine_id: u64,
    last_ms: u64,
    seq: u64,
}

impl Snowflake {
    fn new(machine_id: u64) -> Self {
        assert!(machine_id < (1 << MACHINE_BITS), "machine_id must fit in 10 bits");
        Self { machine_id, last_ms: 0, seq: 0 }
    }

    fn now_ms() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64 - EPOCH_MS
    }

    fn next_id(&mut self) -> u64 {
        let mut now = Self::now_ms();

        while now < self.last_ms {
            now = Self::now_ms();
        }

        if now == self.last_ms {
            self.seq = (self.seq + 1) & MAX_SEQ;
            if self.seq == 0 {
                while now <= self.last_ms {
                    now = Self::now_ms();
                }
            }
        } else {
            self.seq = 0;
        }

        self.last_ms = now;
        (now << (MACHINE_BITS + SEQ_BITS)) | (self.machine_id << SEQ_BITS) | self.seq
    }
}

fn main() {
    let mut gen_id = Snowflake::new(1);
    for _ in 0..3 {
        println!("{}", gen_id.next_id());
    }
}
