use {
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, hash::Hash},
};

pub fn remove_first_occurrence_from_vec<T: PartialEq>(
    vec: &mut Vec<T>,
    data: &T,
) -> Option<T> {
    vec.iter().position(|x| x == data).map(|i| vec.remove(i))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Layering<T: Copy + Eq + Hash> {
    layers: Vec<Vec<T>>,
    depth: HashMap<T, usize>,
}

impl<T: Copy + Eq + Hash> Default for Layering<T> {
    fn default() -> Self {
        Self {
            layers: vec![],
            depth: HashMap::new(),
        }
    }
}

impl<T> Layering<T>
where
    T: Copy + Eq + Hash,
{
    fn layer(&mut self, layer: usize) -> &mut Vec<T> {
        while self.layers.len() <= layer {
            self.layers.push(Vec::new());
        }
        &mut self.layers[layer]
    }

    pub fn len(&self) -> usize {
        self.layers.iter().map(|layer| layer.len()).sum()
    }

    pub fn bisect_layer(
        &self,
        data: &T,
        layer: usize,
        cmp: impl Fn(&T, &T) -> std::cmp::Ordering,
    ) -> usize {
        if self.layers.len() <= layer {
            return 0;
        }
        bisection::bisect_right_by(&self.layers[layer], |other| {
            cmp(other, data)
        })
    }

    pub fn put_in_layer(&mut self, data: T, layer: usize, pos: usize) {
        self.layer(layer).insert(pos, data);
        self.depth.insert(data, layer);
    }

    pub fn remove_from_layer(&mut self, data: &T, layer: usize) -> bool {
        if layer >= self.layers.len() {
            return false;
        }
        remove_first_occurrence_from_vec(&mut self.layers[layer], data)
            .map(|_| self.depth.remove(data))
            .is_some()
    }

    pub fn position(&self, data: &T) -> Option<usize> {
        self.depth.get(data).and_then(|&depth| {
            self.layers[depth]
                .iter()
                .position(|item| item == data)
                .map(|pos| {
                    pos + self
                        .layers
                        .iter()
                        .map(|layer| layer.len())
                        .take(depth)
                        .sum::<usize>()
                })
        })
    }

    pub fn depth(&self, data: &T) -> Option<usize> {
        self.depth.get(data).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.layers.iter().flat_map(|layer| layer.iter())
    }

    pub fn contains(&self, data: &T) -> bool {
        self.depth.contains_key(data)
    }
}
