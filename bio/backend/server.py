import os
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import parse_qs, quote

DEFAULT_NAME = "Anonymous User"
MAX_NAME_CHARS = 50
MAX_BODY_BYTES = 4096
FRONTEND_DIR = os.environ.get("FRONTEND_DIR", "../frontend")


class Handler(SimpleHTTPRequestHandler):
    def do_POST(self) -> None:
        if self.path != "/submit":
            self.send_error(404)
            return

        try:
            length = int(self.headers.get("Content-Length", 0))
        except ValueError:
            self.send_error(400, "Invalid Content-Length")
            return
        if length > MAX_BODY_BYTES:
            self.send_error(413, "Request body too large")
            return

        body = self.rfile.read(length).decode("utf-8", errors="replace")
        form = parse_qs(body)

        name = form.get("name", [""])[0].strip() or DEFAULT_NAME
        name = name[:MAX_NAME_CHARS]

        self.send_response(303)
        self.send_header("Location", f"/linktree.html?name={quote(name, safe='')}")
        self.send_header("Content-Length", "0")
        self.end_headers()


def main() -> None:
    handler = partial(Handler, directory=FRONTEND_DIR)
    with ThreadingHTTPServer(("127.0.0.1", 3000), handler) as server:
        print(
            f"Listening on http://{server.server_address[0]}:{server.server_address[1]}"
        )
        server.serve_forever()


if __name__ == "__main__":
    main()
