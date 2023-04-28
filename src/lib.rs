use std::{thread, time::Duration};

pub struct Dilation(u8);

impl Dilation {
    pub fn new(per: u8) -> Self {
        Dilation(per)
    }
}

impl From<u8> for Dilation {
    fn from(u: u8) -> Self {
        Dilation::new(u)
    }
}

impl Default for Dilation {
    fn default() -> Self {
        Dilation(0)
    }
}

pub trait Waiter {
    fn wait(&self, dur: Duration);
}

pub trait Dilator {
    fn dilate(&mut self, dilation: Dilation);
}

#[derive(Default)]
pub struct DilatingClock {
    dilation: Dilation,
}

impl DilatingClock {
    pub fn new(dilation: Dilation) -> Self {
        DilatingClock { dilation }
    }
}

impl Dilator for DilatingClock {
    fn dilate(&mut self, dilation: Dilation) {
        self.dilation = dilation;
    }
}

impl Waiter for DilatingClock {
    fn wait(&self, duration: Duration) {
        let d = duration.mul_f64(self.dilation.0 as f64 / 100_f64);
        thread::sleep(d);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::time::Instant;

    #[test]
    fn dilation() {
        let d = DilatingClock::new(200.into());
        let now = Instant::now();
        d.wait(Duration::from_secs(1));
        let then = Instant::now();
        assert!(then.duration_since(now) >= Duration::from_secs(2));
    }
}
