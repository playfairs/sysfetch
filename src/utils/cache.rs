use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub value: String,
    pub timestamp: Instant,
    pub ttl: Duration,
}

impl CacheEntry {
    pub fn new(value: String, ttl: Duration) -> Self {
        Self {
            value,
            timestamp: Instant::now(),
            ttl,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        self.timestamp.elapsed() > self.ttl
    }
    
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }
}

pub struct MemoryCache {
    entries: HashMap<String, CacheEntry>,
    default_ttl: Duration,
}

impl MemoryCache {
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            default_ttl,
        }
    }
    
    pub fn get(&self, key: &str) -> Option<String> {
        self.entries.get(key).and_then(|entry| {
            if entry.is_valid() {
                Some(entry.value.clone())
            } else {
                None
            }
        })
    }
    
    pub fn set(&mut self, key: &str, value: String) {
        self.set_with_ttl(key, value, self.default_ttl);
    }
    
    pub fn set_with_ttl(&mut self, key: &str, value: String, ttl: Duration) {
        let entry = CacheEntry::new(value, ttl);
        self.entries.insert(key.to_string(), entry);
    }
    
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.entries.remove(key).map(|entry| entry.value)
    }
    
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    
    pub fn cleanup_expired(&mut self) {
        self.entries.retain(|_, entry| entry.is_valid());
    }
    
    pub fn size(&self) -> usize {
        self.entries.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    
    pub fn keys(&self) -> Vec<String> {
        self.entries.keys().cloned().collect()
    }
    
    pub fn get_or_set<F>(&mut self, key: &str, provider: F) -> String
    where
        F: FnOnce() -> String,
    {
        if let Some(value) = self.get(key) {
            value
        } else {
            let value = provider();
            self.set(key, value.clone());
            value
        }
    }
    
    pub fn get_or_set_with_ttl<F>(&mut self, key: &str, ttl: Duration, provider: F) -> String
    where
        F: FnOnce() -> String,
    {
        if let Some(value) = self.get(key) {
            value
        } else {
            let value = provider();
            self.set_with_ttl(key, value.clone(), ttl);
            value
        }
    }
}

impl Default for MemoryCache {
    fn default() -> Self {
        Self::new(Duration::from_secs(60))
    }
}

pub struct CacheManager {
    memory_cache: MemoryCache,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            memory_cache: MemoryCache::default(),
        }
    }
    
    pub fn get(&mut self, key: &str) -> Option<String> {
        self.memory_cache.get(key)
    }
    
    pub fn set(&mut self, key: &str, value: String) {
        self.memory_cache.set(key, value);
    }
    
    pub fn get_or_set<F>(&mut self, key: &str, provider: F) -> String
    where
        F: FnOnce() -> String,
    {
        self.memory_cache.get_or_set(key, provider)
    }
    
    pub fn cleanup(&mut self) {
        self.memory_cache.cleanup_expired();
    }
    
    pub fn clear(&mut self) {
        self.memory_cache.clear();
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

pub static mut GLOBAL_CACHE: Option<CacheManager> = None;
static mut CACHE_INIT: std::sync::Once = std::sync::Once::new();

pub fn get_global_cache() -> &'static mut CacheManager {
    unsafe {
        CACHE_INIT.call_once(|| {
            GLOBAL_CACHE = Some(CacheManager::new());
        });
        GLOBAL_CACHE.as_mut().unwrap()
    }
}

pub fn get_global_cache_with_file_cache(_cache_dir: String) -> &'static mut CacheManager {
    // for now, just return the same global cache.
    // using proper impl, this would set up file-based caching
    get_global_cache()
}