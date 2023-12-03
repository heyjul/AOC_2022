#[derive(Debug, Default, PartialEq)]
pub struct Number {
    pub value: u32,
    pub start: usize,
    pub stop: usize,
}

//55 - 57,58
impl Number {
    pub fn is_adjacent(&self, p: usize, w: usize) -> bool {
        (self.start..=self.stop).any(|x| {
            (p.checked_sub(w + 1).unwrap_or(0)..=p.checked_sub(w - 1).unwrap_or(0)).contains(&x)
                || x == p.checked_sub(1).unwrap_or(0)
                || x == p + 1
                || ((p + w - 1)..=(p + w + 1)).contains(&x)
        })
    }
}
