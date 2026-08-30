use std::collections::VecDeque;

/// Ordered one-shot game commands waiting for a fixed-tick boundary.
pub struct GameCommandQueue<C> {
    commands: VecDeque<C>,
}

impl<C> GameCommandQueue<C> {
    /// Create an empty command queue.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a command after all commands already waiting for a fixed tick.
    pub fn push(&mut self, command: C) {
        self.commands.push_back(command);
    }

    /// Number of commands still waiting for consumption.
    #[must_use]
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Whether no commands are waiting for consumption.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Drain every waiting command in insertion order.
    pub fn drain(&mut self) -> impl Iterator<Item = C> + '_ {
        self.commands.drain(..)
    }
}

impl<C> Default for GameCommandQueue<C> {
    fn default() -> Self {
        Self {
            commands: VecDeque::new(),
        }
    }
}

#[cfg(test)]
mod test;
