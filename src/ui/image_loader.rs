//! Custom HTTP Image Loader for egui with configurable timeout
//!
//! Uses reqwest with a longer timeout than egui's default ehttp loader,
//! which helps when loading images from slow external services.

use eframe::egui;
use egui::load::{BytesLoadResult, BytesLoader, BytesPoll, LoadError};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Loading state for an image
#[derive(Clone)]
enum LoadState {
    Loading,
    Loaded(Arc<[u8]>),
    Failed(String),
}

/// Custom HTTP loader with longer timeout (30 seconds)
pub struct CustomHttpLoader {
    cache: Arc<Mutex<HashMap<String, LoadState>>>,
}

impl CustomHttpLoader {
    /// Create a new loader
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Install this loader into an egui context
    pub fn install(ctx: &egui::Context) {
        ctx.add_bytes_loader(Arc::new(Self::new()));
    }
}

impl BytesLoader for CustomHttpLoader {
    fn id(&self) -> &str {
        egui::generate_loader_id!(CustomHttpLoader)
    }
    
    fn load(&self, ctx: &egui::Context, uri: &str) -> BytesLoadResult {
        // Only handle http/https URLs
        if !uri.starts_with("http://") && !uri.starts_with("https://") {
            return Err(LoadError::NotSupported);
        }
        
        // Check cache first
        {
            let cache = self.cache.lock().unwrap();
            if let Some(state) = cache.get(uri) {
                return match state {
                    LoadState::Loading => Ok(BytesPoll::Pending { size: None }),
                    LoadState::Loaded(bytes) => Ok(BytesPoll::Ready {
                        size: None, // Size is determined after image decoding
                        bytes: egui::load::Bytes::Shared(bytes.clone()),
                        mime: None,
                    }),
                    LoadState::Failed(err) => Err(LoadError::Loading(err.clone())),
                };
            }
        }
        
        // Mark as loading
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(uri.to_string(), LoadState::Loading);
        }
        
        // Spawn download task
        let uri = uri.to_string();
        let cache = self.cache.clone();
        let ctx = ctx.clone();
        
        std::thread::spawn(move || {
            // Use blocking reqwest client with 30s timeout
            let result = reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .and_then(|client| client.get(&uri).send())
                .and_then(|response| response.bytes());
            
            let state = match result {
                Ok(bytes) => {
                    tracing::info!("Loaded image: {} ({} bytes)", uri, bytes.len());
                    LoadState::Loaded(bytes.to_vec().into())
                }
                Err(e) => {
                    tracing::warn!("Failed to load image {}: {}", uri, e);
                    LoadState::Failed(e.to_string())
                }
            };
            
            {
                let mut cache = cache.lock().unwrap();
                cache.insert(uri, state);
            }
            
            ctx.request_repaint();
        });
        
        Ok(BytesPoll::Pending { size: None })
    }
    
    fn forget(&self, uri: &str) {
        let mut cache = self.cache.lock().unwrap();
        cache.remove(uri);
    }
    
    fn forget_all(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    fn byte_size(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.values().map(|state| {
            match state {
                LoadState::Loaded(bytes) => bytes.len(),
                _ => 0,
            }
        }).sum()
    }
}
