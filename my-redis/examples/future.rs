use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            println!("時は来た");
            Poll::Ready("done")
        } else {
            cx.waker().wake_by_ref();
            println!("時を待っている");
            Poll::Pending
        }
    }
}

enum MainFuture {
    Init,
    Running(Delay),
    Terminated,
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        use MainFuture::*;
        loop {
            match *self {
                Init => {
                    let when = Instant::now() + Duration::from_millis(10);
                    let future = Delay { when };
                    *self = Running(future);
                }
                Running(ref mut delay) => {
                    match Pin::new(delay).poll(cx) {
                        Poll::Ready(out) => {
                            assert_eq!(out, "done");
                            *self = Terminated;
                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                    }
                }
                Terminated => {
                    panic!("future polled after completion")
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let future = MainFuture::Init;
    future.await;
}
