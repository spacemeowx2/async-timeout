use tokio::time::{Instant, Duration, Sleep, sleep};
use std::{io, pin::Pin, task::{Poll, Context}, future::Future, sync::{Arc, Mutex}};

#[derive(Debug)]
pub struct VisitorTimeout {
    last_visit: Arc<Mutex<Instant>>,
    timeout: Duration,
    timer: Sleep,
}

#[derive(Debug, Clone)]
pub struct Visitor {
    last_visit: Arc<Mutex<Instant>>,
}

impl VisitorTimeout {
    pub fn new(timeout: Duration) -> (VisitorTimeout, Visitor)
    {
        let last_visit = Arc::new(Mutex::new(Instant::now()));
        (VisitorTimeout {
            last_visit: last_visit.clone(),
            timeout,
            timer: sleep(timeout),
        }, Visitor {
            last_visit
        })
    }
    pub fn poll_timeout(&mut self,  cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        loop {
            if let Poll::Pending = Pin::new(&mut self.timer).poll(cx) {
                return Poll::Pending;
            }
            let elapsed = self.last_visit.lock().unwrap().elapsed();
            if elapsed > self.timeout {
                return Poll::Ready(Err(io::ErrorKind::TimedOut.into()))
            } else {
                self.timer = sleep(self.timeout - elapsed);
            }
        }
    }
}

impl Future for VisitorTimeout {
    type Output = io::Result<()>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.poll_timeout(cx)
    }
}

impl Visitor {
    pub fn visit(&self) {
        *self.last_visit.lock().unwrap() = Instant::now();
    }
}

#[derive(Debug)]
pub struct Timeout {
    last_visit: Instant,
    timeout: Duration,
    timer: Sleep,
}

impl Timeout {
    pub fn new(timeout: Duration) -> Timeout
    {
        Timeout {
            last_visit: Instant::now(),
            timeout,
            timer: sleep(timeout),
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
                self.timer = sleep(self.timeout - elapsed);
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
