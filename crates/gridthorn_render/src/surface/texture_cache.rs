use gridthorn_assets::TextureAsset;

/// Device-local resources live only while their decoded identity occurs in a frame.
pub(super) struct TextureCache<T> {
    entries: Vec<(TextureAsset, T)>,
}

impl<T> TextureCache<T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn retain(&mut self, textures: &[&TextureAsset]) {
        self.entries.retain(|(asset, _)| {
            textures
                .iter()
                .any(|texture| asset.shares_data_with(texture))
        });
    }

    pub fn get_or_insert(
        &mut self,
        asset: &TextureAsset,
        create: impl FnOnce() -> T,
    ) -> (&T, bool) {
        if let Some(index) = self
            .entries
            .iter()
            .position(|(stored, _)| stored.shares_data_with(asset))
        {
            return (&self.entries[index].1, false);
        }
        self.entries.push((asset.clone(), create()));
        (&self.entries.last().expect("inserted texture").1, true)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn bytes(&self) -> usize {
        self.entries
            .iter()
            .map(|(asset, _)| asset.rgba8().len())
            .sum()
    }
}

#[cfg(test)]
#[path = "texture_cache/test/mod.rs"]
mod test;
