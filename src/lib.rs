extern crate vpsearch;

use std::collections::HashMap;

trait MetricPoint {
    fn distance(&self, other: &Self) -> f64;
}

#[derive(Debug, Copy, Clone)]
struct Index {
    index: usize,
}

impl Index {
    fn new(i: usize) -> Index {
        Index { index: i }
    }
}

impl<T: MetricPoint> vpsearch::MetricSpace for Index {
    type UserData = HashMap<usize, T>;
    type Distance = f64;

    fn distance(&self, other: &Self, points: &Self::UserData) -> Self::Distance {
        let p1 = &points[&self.index];
        let p2 = &points[&other.index];
        p1.distance(&p2)
    }
}

struct SearchContainer<T: MetricPoint> {
    points: HashMap<usize, T>,
    vptree: vpsearch::Tree<Index, (), ()>,
}

impl<T: MetricPoint> SearchContainer<T> {
    fn new(mut index_points: Vec<T>) -> SearchContainer<T> {
        let mut points: HashMap<usize, T> = HashMap::new();
        for i in (0..index_points.len()).rev() {
            let p = index_points.pop().unwrap();
            points.insert(i, p);
        }

        let keys: Vec<Index<T>> = (0..index_points.len()).map(Index::new).collect();

        let tree = vpsearch::Tree::new_with_user_data_ref(&keys, &points);

        SearchContainer {
            points,
            vptree: tree,
        }
    }

    fn find_nearest(&mut self, p: T) -> (T, &T) {
        let index = self.points.len();
        self.points.insert(index, p);
        let (found_index, _) = self.vptree.find_nearest(&Index::new(index), &self.points);
        let p = self.points.remove(&index).unwrap();
        (p, self.points.get(&found_index).unwrap())
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
