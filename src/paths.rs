pub struct AllPaths<T: warp::Filter> {
    path: T,
}

impl<T> AllPaths<T>
where
    T: std::marker::Sync + warp::Filter + warp::Reply + std::clone::Clone,
{
    pub fn new(bf: T) -> Self {
        Self { path: bf }
    }

    pub(crate) fn run(mut self, port: u16) -> () {
        let server = warp::serve(self.path).run(([127, 0, 0, 1], 3000));
    }
}
