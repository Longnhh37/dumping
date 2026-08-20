pub enum Frame {
    Simple(String), // +OK
    Error(String), // -ERR ...
    Integer(i64), // :42
    Bulk(Option<Vec<u8>>), // $3\r\nfoo or $-1 (null)
    Array(Option<Vec<Frame>>), // *2\r\n... or $-1 (null)
}
