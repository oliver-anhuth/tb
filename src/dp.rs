//! Dynamic Programming

pub struct Tableau<T> {
    row_size: usize,
    data: Vec<T>,
}

impl<T> Tableau<T> {
    pub fn new(rows: usize, cols: usize) -> Self
    where
        T: Default,
    {
        let mut tableau = Self {
            row_size: cols,
            data: Vec::new(),
        };
        tableau.data.resize_with(rows * cols, Default::default);
        tableau
    }

    pub fn row(&self, row: usize) -> &[T] {
        let start = row * self.row_size;
        let end = start + self.row_size;
        &self.data[start..end]
    }

    pub fn row_mut(&mut self, row: usize) -> &mut [T] {
        let start = row * self.row_size;
        let end = start + self.row_size;
        &mut self.data[start..end]
    }
}

impl<T> std::ops::Index<usize> for Tableau<T> {
    type Output = [T];
    fn index(&self, row: usize) -> &Self::Output {
        self.row(row)
    }
}

impl<T> std::ops::IndexMut<usize> for Tableau<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        self.row_mut(row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tableau() {
        let mut d = Tableau::new(10, 10);
        d[0][0] = 42;
        assert_eq!(d[0][0], 42);
    }

    #[test]
    #[should_panic]
    fn test_tableau_oob() {
        let mut d = Tableau::new(10, 10);
        d[100][1] = 42;
    }
}
