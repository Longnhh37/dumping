#!/usr/bin/env zsh
# File: scripts/basic_test.sh

# Navigate to workspace root (parent of scripts/)
SCRIPTPATH="${0:A:h}"
cd "$SCRIPTPATH/.."

framework=${1:-actix}

case $framework in
  actix)
    echo "Building and running Actix server..."
    cargo build -p to-do-actix-server && cargo run -p to-do-actix-server &
    ;;
  axum)
    echo "Building and running Axum server..."
    cargo build -p to-do-axum-server && cargo run -p to-do-axum-server &
    ;;
  hyper)
    echo "Building and running Hyper server..."
    cargo build -p to-do-hyper-server && cargo run -p to-do-hyper-server &
    ;;
  rocket)
    echo "Building and running Rocket server..."
    cargo build -p to-do-rocket-server && cargo run -p to-do-rocket-server &
    ;;
  *)
    echo "Unknown framework: $framework"
    echo "Usage: $0 [actix|axum|hyper|rocket]"
    exit 1
    ;;
esac

PID=$!
sleep 1

echo '{}' > tasks.json
rm -f output.txt

xh POST :8080/api/v1/create title=writing status=Pending >> output.txt
xh POST :8080/api/v1/create title=coding status=Pending >> output.txt
xh DELETE :8080/api/v1/delete/coding >> output.txt

kill $PID
