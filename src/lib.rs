use tokio::time::{Instant, Duration, Delay, delay_for};
use std::{io, pin::Pin, task::{Poll, Context}, future::Future};

#[derive(Debug)]
pub struct Timeout {
    last_visit: Instant,
    timeout: Duration,
    timer: Delay,
}

impl Timeout {
    pub fn new(timeout: Duration) -> Timeout
    {
        Timeout {
            last_visit: Instant::now(),
            timeout,
            timer: delay_for(timeout),
        }
    }
    pub fn visit(&mut self) {
        self.last_visit = Instant::now();
    }
    pub fn poll_timeout(&mut self,  cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        loop {
            if let Poll::Pending = Pin::new(&mut self.timer).poll(cx) {
                return Poll::Pending;
            }
            let elapsed = self.last_visit.elapsed();
            if elapsed > self.timeout {
                return Poll::Ready(Err(io::ErrorKind::TimedOut.into()))
            } else {
                self.timer = delay_for(self.timeout - elapsed);
            }
        }
    }
}

impl Future for Timeout {
    type Output = io::Result<()>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.poll_timeout(cx)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
