// ----------------------------------------------------------------------------
// Newtype pattern
// ----------------------------------------------------------------------------

// Without newtypes — easy to mix up:
fn create_user(name: String, email: String, age: u32, employee_id: u32) {}
// create_user(name, email, age, id);  — but what if we swap age and id?
// create_user(name, email, id, age);  — COMPILES FINE, BUG

// With newtypes — the compiler catches mistakes:
struct UserName(String);
struct Email(String);
struct Age(u32);
struct EmployeeId(u32);

fn create_user_2(name: UserName, email: Email, age: Age, id: EmployeeId) {}
// create_user(name, email, EmployeeId(42), Age(30));
// Compile error: expected Age, got EmployeeId
