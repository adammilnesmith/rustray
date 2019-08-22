use std::sync::{Arc, RwLock};
use std::thread;

macro_rules! vec_no_clone {
    ( $val:expr; $n:expr ) => {{
        let result: Vec<_> = std::iter::repeat_with(|| $val).take($n).collect();
        result
    }};
}

pub struct Canvas<T> {
    pub x_size: usize,
    pub y_size: usize,
    pub canvas: Vec<RwLock<T>>, //TODO: modify this to use AtomicPtr later
}

#[inline]
fn to_index(x: usize, y: usize, x_size: usize) -> usize {
    y * x_size + x
}

impl<T> Canvas<T>
where
    T: Clone,
{
    #[inline]
    fn new(x_size: usize, y_size: usize, canvas: Vec<RwLock<T>>) -> Canvas<T> {
        Canvas {
            x_size,
            y_size,
            canvas,
        }
    }

    #[inline]
    pub fn new_blank(x_size: usize, y_size: usize, init_value: T) -> Canvas<T> {
        Canvas::new(
            x_size,
            y_size,
            vec_no_clone![RwLock::new(init_value.clone()); x_size * y_size],
        )
    }

    pub fn x_size(&self) -> usize {
        self.x_size
    }

    pub fn y_size(&self) -> usize {
        self.y_size
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> T {
        self.canvas
            .get(to_index(x, y, self.x_size))
            .unwrap()
            .read()
            .unwrap()
            .clone()
    }

    pub fn update_pixel<F>(&self, x: usize, y: usize, update: F) -> T
    where
        F: Fn(T) -> T,
    {
        let mut value = self
            .canvas
            .get(to_index(x, y, self.x_size))
            .unwrap()
            .write()
            .unwrap();
        *value = update((*value).clone());
        (*value).clone()
    }
}

impl<T> Into<Vec<T>> for Canvas<T>
where
    T: Clone,
{
    fn into(self) -> Vec<T> {
        self.canvas
            .iter()
            .map(|v| v.read().unwrap().clone())
            .collect()
    }
}

impl<T> Into<Vec<T>> for &Canvas<T>
where
    T: Clone,
{
    fn into(self) -> Vec<T> {
        self.canvas
            .iter()
            .map(|v| v.read().unwrap().clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blank_canvas() {
        assert_eq!(Canvas::new_blank(16, 9, 0.0).canvas.len(), 16 * 9);
    }

    #[test]
    fn test_multi_threaded_access() {
        let canvas = Arc::new(Canvas::new_blank(1, 1, 0.0));

        let mut handles = vec![];

        for _ in 0..10 {
            let canvas = Arc::clone(&canvas);
            let handle = thread::spawn(move || canvas.update_pixel(0, 0, |x| x + 1.0));
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        /* TODO?
        (0..10)
        .map(|_| Arc::clone(&canvas))
        .map(|canvas| thread::spawn(move || canvas.update_pixel(0, 0, |x| x + 1.0)))
        .map(|handle| handle.join().unwrap());
        */

        assert_eq!(canvas.read_pixel(0, 0), 10.0);
    }

    #[test]
    fn test_into_vec() {
        let canvas = Canvas::new_blank(2, 1, 0.0);
        let as_vec: Vec<f64> = (&canvas).into();
        assert_eq!(as_vec, vec![0.0, 0.0]);
        canvas.update_pixel(0, 0, |_| 1.0);
        assert_eq!(canvas.read_pixel(0, 0), 1.0);
        assert_eq!(canvas.read_pixel(1, 0), 0.0);
        assert_eq!(as_vec, vec![0.0, 0.0]);
    }
}
