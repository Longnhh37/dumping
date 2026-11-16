pub mod create;
pub mod get;

use create::SaveOne;
use get::GetByEmail;

// When the corresponding transaction traits are added, uncomment them in both
// AuthStore bounds below:
// get::GetById, get::GetAll, update::UpdateOne, delete::DeleteOne
pub trait AuthStore: SaveOne + GetByEmail {}

// impl<T> AuthStore for T
// where
//     T: SaveOne + GetByEmail + get::GetById + get::GetAll + update::UpdateOne + delete::DeleteOne,
// {}
impl<T> AuthStore for T where T: SaveOne + GetByEmail {}
