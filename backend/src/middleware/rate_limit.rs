use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

pub struct RateLimiter {
    buckets: Mutex<HashMap<String, TokenBucket>>,
}

struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
    capacity: f64,
    refill_rate: f64,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32) -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
        }
    }

    pub fn allow(&self, key: &str) -> bool {
        let mut buckets = self.buckets.lock().unwrap();
        let bucket = buckets.entry(key.to_string()).or_insert_with(|| {
            let now = Instant::now();
            TokenBucket {
                tokens: 30.0,
                last_refill: now,
                capacity: 30.0,
                refill_rate: 0.5, // 30 per minute
            }
        });

        let now = Instant::now();
        let elapsed = now.duration_since(bucket.last_refill);
        let refill = elapsed.as_secs_f64() * bucket.refill_rate;
        bucket.tokens = (bucket.tokens + refill).min(bucket.capacity);
        bucket.last_refill = now;

        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}
