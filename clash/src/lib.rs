pub const MAX_PLAYERS: usize = 6;
pub const HELLO: &str = "Hello World";
// NOTE: We can considering using the newtype pattern here to avoid the possiblity of mixing up these id types,
//       but it adds a lot of boilerplate and I'm not sure that it's actually worth it at this point.
pub type PlayerId = u32;
pub type LobbyId = u32;
