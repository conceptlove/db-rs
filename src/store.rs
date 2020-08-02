pub struct Query();

trait Store {
    fn new() -> Self;
    fn adding(&mut self, item: Fact);
    fn removing(&mut self, item: Fact);

    fn add<T: Into<Fact>>(&mut self, item: T) -> &mut Self {
        self.adding(item.into());
        self
    }

    fn remove<T: Into<Fact>>(&mut self, item: T) -> &mut Self {
        self.removing(item.into());
        self
    }
}
