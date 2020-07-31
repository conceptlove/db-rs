pub trait Machine<In, Out>
where
    Self: Sized,
{
    fn next(&self, msg: In) -> (Self, Out);
}

// impl<In, Out> std::ops::Add<Out> for dyn Machine<In, Out> {}

pub trait Reducer<In>: Machine<In, Self>
where
    Self: Sized,
{
    fn update(&self, msg: In) -> Self;
}

impl<In, R> Machine<In, Self> for R
where
    R: Reducer<In> + Clone,
{
    fn next(&self, msg: In) -> (Self, Self) {
        let state = self.update(msg);
        (state.clone(), state)
    }
}

pub fn unit(n: &i32) -> i32 {
    if *n < 0 {
        -1
    } else {
        1
    }
}

/// Trying to capture the essence of constructing an i32 from a series of digits.
impl Reducer<u8> for i32 {
    fn update(&self, digit: u8) -> Self {
        self * 10 + (digit as i32) * unit(self)
    }
}
