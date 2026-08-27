/// The version of the Gridthorn SDK used to compile the game.
#[must_use]
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod test;
