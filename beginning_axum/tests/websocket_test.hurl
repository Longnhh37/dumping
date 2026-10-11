# 1. Request normal HTTP thường (without header Upgrade) -> axum denies upgrade
GET http://localhost:3000/ws
HTTP 400

# 2. Valid Handshake WebSocket -> server returns 101 Switching Protocols
GET http://localhost:3000/ws
Connection: Upgrade
Upgrade: websocket
Sec-WebSocket-Version: 13
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==
[Options]
max-time: 3
HTTP 101
[Asserts]
header "Upgrade" == "websocket"
header "Sec-WebSocket-Accept" exists
