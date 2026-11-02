// ----------------------------------------------------------------------------
// Type-State
// ----------------------------------------------------------------------------

// Problem: A network connection that must be:
// 1. Created
// 2. Connected
// 3. Authenticated
// 4. Then used for requests
// Calling request() before authenticate() should be a COMPILE error.
//
// Make sure method take 'self' as parameter to invalidate old state

// Type-state markers (zero_sized types)
struct Disconnected;
struct Connected;
struct Authenticated;

// Connection parameterized by state
struct Connection<State> {
    address: String,
    _state: std::marker::PhantomData<State>,
}

// Only Disconnected connections can connect
impl Connection<Disconnected> {
    fn new(address: &str) -> Self {
        Connection {
            address: address.to_string(),
            _state: std::marker::PhantomData,
        }
    }

    fn connect(self) -> Connection<Connected> {
        println!("Connecting to {}...", self.address);
        Connection {
            address: self.address,
            _state: std::marker::PhantomData,
        }
    }
}

// Only Connected connections can authenticate
impl Connection<Connected> {
    fn authenticate(self, _token: &str) -> Connection<Authenticated> {
        println!("Authenticating...");
        Connection {
            address: self.address,
            _state: std::marker::PhantomData,
        }
    }
}

// Only Authenticated connections can make requests
impl Connection<Authenticated> {
    fn request(&self, path: &str) -> String {
        format!("GET {} from {}", path, self.address)
    }
}

fn example() {
    let conn = Connection::new("api.example.com");
    let conn = conn.connect();
    let conn = conn.authenticate("secret-token");
    let response = conn.request("/data");
    println!("{response}");
}
