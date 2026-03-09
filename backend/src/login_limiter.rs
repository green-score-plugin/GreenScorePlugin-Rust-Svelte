use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

const MAX_ATTEMPTS: u32 = 5;
const BLOCK_DURATION: Duration = Duration::from_secs(15 * 60); // 15 minutes
const MAX_ENTRIES: usize = 50_000;          // limite absolue de la map
const CLEANUP_INTERVAL: Duration = Duration::from_secs(5 * 60); // cleanup toutes les 5 min

struct AttemptRecord {
    count: u32,
    first_attempt: Instant,
    blocked_at: Option<Instant>,
}

#[derive(Clone)]
pub struct LoginLimiter {
    map: Arc<DashMap<String, AttemptRecord>>,
}

impl LoginLimiter {
    pub fn new() -> Self {
        let limiter = Self {
            map: Arc::new(DashMap::new()),
        };

        // Spawn du cleanup périodique
        let map_clone = limiter.map.clone();
        tokio::spawn(async move {
            loop {
                sleep(CLEANUP_INTERVAL).await;
                map_clone.retain(|_, record| {
                    // Garder uniquement les entrées encore actives (bloquées ou dans la fenêtre)
                    if let Some(blocked_at) = record.blocked_at {
                        return blocked_at.elapsed() < BLOCK_DURATION;
                    }
                    record.first_attempt.elapsed() < BLOCK_DURATION
                });
            }
        });

        limiter
    }

    /// Retourne true si l'IP est bloquée, false sinon.
    pub fn is_blocked(&self, ip: &str) -> bool {
        if let Some(mut record) = self.map.get_mut(ip) {
            if let Some(blocked_at) = record.blocked_at {
                if blocked_at.elapsed() >= BLOCK_DURATION {
                    record.count = 0;
                    record.blocked_at = None;
                    record.first_attempt = Instant::now();
                    return false;
                }
                return true;
            }
            if record.first_attempt.elapsed() >= BLOCK_DURATION {
                record.count = 0;
                record.first_attempt = Instant::now();
            }
        }
        false
    }

    /// Enregistre une tentative échouée. Retourne le nombre de tentatives restantes.
    pub fn record_failure(&self, ip: &str) -> u32 {
        // Si la map est pleine, on refuse silencieusement d'ajouter de nouvelles entrées
        // (les IPs inconnues ne sont pas bloquées mais on préserve la RAM)
        if !self.map.contains_key(ip) && self.map.len() >= MAX_ENTRIES {
            return MAX_ATTEMPTS; // on laisse passer, mieux que crasher
        }

        let mut record = self.map.entry(ip.to_string()).or_insert_with(|| AttemptRecord {
            count: 0,
            first_attempt: Instant::now(),
            blocked_at: None,
        });

        if record.first_attempt.elapsed() >= BLOCK_DURATION {
            record.count = 0;
            record.first_attempt = Instant::now();
        }

        record.count += 1;

        if record.count >= MAX_ATTEMPTS {
            record.blocked_at = Some(Instant::now());
            return 0;
        }

        MAX_ATTEMPTS - record.count
    }

    /// Réinitialise les tentatives d'une IP après un succès.
    pub fn reset(&self, ip: &str) {
        self.map.remove(ip);
    }

    /// Retourne le temps de blocage restant en secondes (0 si non bloqué).
    pub fn remaining_block_secs(&self, ip: &str) -> u64 {
        if let Some(record) = self.map.get(ip) {
            if let Some(blocked_at) = record.blocked_at {
                let elapsed = blocked_at.elapsed();
                if elapsed < BLOCK_DURATION {
                    return (BLOCK_DURATION - elapsed).as_secs();
                }
            }
        }
        0
    }
}
