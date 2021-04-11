fn main() {
    println!("Hello, world!");
}

struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    pub fn new(iter: O) -> Self {
        Self {
            outer: iter,
            inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(item) = inner_iter.next() {
                    return Some(item);
                }
                self.inner = None;
            }

            let next_inner_iter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_iter);
        }
    }
}
