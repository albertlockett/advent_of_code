
pub struct Grid<T> {
    data: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self
    where
        T: Clone,
    {
        let data = vec![vec![fill; width]; height];
        Self {
            data,
            width,
            height,
        }
    }

    pub fn from(data: Vec<Vec<T>>) -> Self {
        // TODO empty checks
        let height = data.len();
        let width = data.get(0).map(|row| row.len()).unwrap_or(0);
        Self {
            width,
            height,
            data
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).and_then(|row| row.get(x))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data.get_mut(y).and_then(|row| row.get_mut(x))
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if let Some(row) = self.data.get_mut(y) {
            if let Some(cell) = row.get_mut(x) {
                *cell = value;
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn print(&self) where
    T: std::fmt::Display + Clone {
        for row in &self.data {
            let g = row.iter().map(|e| format!("{}", e));
            let g2 = g.collect::<Vec<String>>();
            println!("{}", g2.join(""));
        }
    }
}