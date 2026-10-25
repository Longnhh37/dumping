#[derive(Debug, Default)]
struct SseEvent {
    id: Option<String>,
    event: Option<String>,
    data: String,
}

fn parse_event(block: &str) -> SseEvent {
    let mut ev = SseEvent::default();
    for line in block.lines() {
        if line.starts_with(':') {
            continue; // comment / heartbeat
        }
        let (field, value) = line.split_once(':').unwrap_or((line, ""));
        let value = value.strip_prefix(' ').unwrap_or(value);
        match field {
            "id" => ev.id = Some(value.to_owned()),
            "event" => ev.event = Some(value.to_owned()),
            "data" => {
                if !ev.data.is_empty() {
                    ev.data.push('\n');
                }
                ev.data.push_str(value);
            }
            _ => {} // retry and unknownd field
        }
    }
    ev
}

fn main() {
    let raw = "id: 1\nevent: tick\ndata: hello\ndata: world\n";
    println!("{:?}", parse_event(raw));
    // SseEvent { id: Some("1"), event: Some("tick"), data: "hello\nworld" }
}
