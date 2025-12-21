

use std::time::Instant;

use futures::channel::mpsc;
use serde::{Serialize, Deserialize};
use smallvec::{SmallVec, smallvec};

#[derive(Debug)]
enum DynamicVec<T> {
    Small(SmallVec<T, 4>),
    Medium(SmallVec<T, 16>),
    Large(SmallVec<T, 64>),
}

impl<T: Clone> DynamicVec<T> {
    fn push(&mut self, item: T) {
        match self {
            DynamicVec::Small(v) if v.len() < 4 => v.push(item),
            DynamicVec::Small(v) => {
                let mut new = DynamicVec::Medium(smallvec![]);
                new.extend(v.drain(..).chain(std::iter::once(item)));
                *self = new;
            }
            DynamicVec::Medium(v) if v.len() < 16 => v.push(item),
            DynamicVec::Medium(v) => {
                let mut new = DynamicVec::Large(smallvec![]);
                new.extend(v.drain(..).chain(std::iter::once(item)));
                *self = new;
            }
            DynamicVec::Large(v) => v.push(item)
        }
    }

    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

fn main1() {
    let mut  v: SmallVec<i32, 4> = smallvec![1,2,3,4];

    println!("inline: {}", v.spilled());

    v.push(5);

    println!("inline: {}", !v.spilled());

    v[0] = v[1] + v[2];

    println!("inline : {:?}", v);

    v.sort();

    println!("sorted: {:?}", v);

}

fn main2() {
    let mut vec = DynamicVec::Small(smallvec![]);
    vec.extend(vec![1,2,3,4,5]);
    println!("{:?}", vec);
}

struct LogEntry {
    timestamp: u64,
    tags: SmallVec<String, 8>,
}

fn parse_log(lines: &[&str]) -> SmallVec<LogEntry, 16> {
    let mut entries : SmallVec<LogEntry, 16> = smallvec![];
    for line in lines {
        let parts: SmallVec<&str, 4> = line.split(',').collect();
        if parts.len() >= 2 {
            let timestamp = parts[0].parse().unwrap_or(0);
            let tags = parts[1..].iter().map(|s|s.to_string()).collect();
            entries.push(LogEntry { timestamp, tags })
        }
    }
    entries
}


#[derive(Serialize, Deserialize, Debug)]
struct Data {
    items: SmallVec<i32, 4>,
}

fn main3() {
    let data = Data {
        items: smallvec![1,2,3,4],
    };
    let serialized = serde_json::to_string(&data).unwrap();

    println!("serialized: {}", serialized);

    let deserialized: Data = serde_json::from_str(&serialized).unwrap();
    println!("deserialized: {:?}", deserialized);

}

fn main4() {
    let logs = vec![
        "1625097600,tag1, tag2",
    ];
    let start = Instant::now();
    let entries = parse_log(&logs);
    let duration = start.elapsed();

    for entry in entries {
        println!("timestamp {} tags {:?}", entry.timestamp, entry.tags);
    }
    println!("{:?}", duration);
}

use tracing::{info_span, event, Level};
use tracing_subscriber::{self, EnvFilter};

fn main5() {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
    let outer_span = info_span!("outer_span", user = "ferris");
    let _outer_enter = outer_span.enter();

    event!(Level::INFO, "enter outer_span");

    {
        let inner_span = info_span!("inner_span");
        let _inner_enter = inner_span.enter();
        event!(Level::DEBUG, message = "inner event", value = 42);
    }

    event!(Level::WARN, "exit warnings");
}

use tracing::instrument;

#[instrument]
fn compute(x : i32, y: i32) -> i32 {
    event!(Level::TRACE, "start calc");
    let result = x + y;
    event!(Level::INFO, result = result);
    result
}

fn main6() {
    tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env()).init();
    compute(5, 10);
}

use futures::executor::block_on;

async fn hello_world() {
    println!("Hello, async world");
}

fn main7() {
    let future = hello_world();
    block_on(future);
}

use futures::{stream::{self, StreamExt}, SinkExt};

async fn count_stream() {
    let mut stream = stream::iter(1..=5);
    while let Some(value) = stream.next().await {
        println!("value: {}", value);
    }
}

fn main8() {
    block_on(count_stream());
}

async fn send_values() {
    let (mut tx, rx) = mpsc::unbounded::<i32>();
    tx.send(42).await.unwrap();
    drop(tx);
}

fn main() {
    block_on(send_values());
}