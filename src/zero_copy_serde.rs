//! Zero-Copy Serialization Framework via rkyv
//!
//! Section 3 of High-Performance System Optimization Projects.
//! Eliminates the JSON serialization tax by implementing zero-copy
//! binary deserialization where network bytes map directly to
//! application structs in ~1.4 nanoseconds.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

// ─── Serialization Framework Comparison ──────────────────────────────────────

/// Serialization framework comparison data (from Section 3.2 benchmarks).
#[derive(Debug, Clone)]
pub struct SerializationBenchmark {
    pub framework: &'static str,
    pub schema_requirement: &'static str,
    pub access_latency: &'static str,
    pub deser_speed: &'static str,
    pub supports_mutation: bool,
    pub suitability: &'static str,
}

/// Get the benchmark comparison table from the research.
pub fn benchmark_table() -> Vec<SerializationBenchmark> {
    vec![
        SerializationBenchmark {
            framework: "JSON (Standard Reflection)",
            schema_requirement: "None",
            access_latency: "N/A (must parse full payload)",
            deser_speed: "Very Slow (high allocations)",
            supports_mutation: false,
            suitability: "Text-based Web APIs, External Facing",
        },
        SerializationBenchmark {
            framework: "FlatBuffers",
            schema_requirement: "External DSL (.fbs)",
            access_latency: "Fast (zero-copy)",
            deser_speed: "Slower on nested (3.96µs read)",
            supports_mutation: false,
            suitability: "Flat Binary Data Analytics",
        },
        SerializationBenchmark {
            framework: "Cap'n Proto",
            schema_requirement: "External DSL",
            access_latency: "Moderate (~250ns)",
            deser_speed: "Fast (zero-copy)",
            supports_mutation: false,
            suitability: "RPC Frameworks, Inter-service",
        },
        SerializationBenchmark {
            framework: "rkyv (Rust Native)",
            schema_requirement: "None (native macros)",
            access_latency: "Extremely Fast (~1.4ns)",
            deser_speed: "Fastest on nested (282ns read)",
            supports_mutation: true,
            suitability: "Complex Structures, HPC Middleware",
        },
    ]
}

// ─── Zero-Copy Data Structures (rkyv-annotated) ─────────────────────────────

/// A microservice API response — zero-copy serializable.
/// When serialized with rkyv, network bytes map directly to this struct.
#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    SerdeSerialize,
    SerdeDeserialize,
    Debug,
    Clone,
    PartialEq,
)]
#[archive(check_bytes)]
pub struct ApiResponse {
    pub status_code: u32,
    pub request_id: String,
    pub timestamp_ms: u64,
    pub headers: Vec<HeaderPair>,
    pub body: ResponseBody,
}

/// Header key-value pair.
#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    SerdeSerialize,
    SerdeDeserialize,
    Debug,
    Clone,
    PartialEq,
)]
#[archive(check_bytes)]
pub struct HeaderPair {
    pub key: String,
    pub value: String,
}

/// Response body variants — demonstrates nested enum support.
#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    SerdeSerialize,
    SerdeDeserialize,
    Debug,
    Clone,
    PartialEq,
)]
#[archive(check_bytes)]
pub enum ResponseBody {
    /// JSON-like structured data
    Structured(StructuredData),
    /// Raw binary payload
    Binary(Vec<u8>),
    /// Empty response
    Empty,
}

/// Deeply nested structured data (analogous to complex JSON API).
#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    SerdeSerialize,
    SerdeDeserialize,
    Debug,
    Clone,
    PartialEq,
)]
#[archive(check_bytes)]
pub struct StructuredData {
    pub entities: Vec<Entity>,
    pub pagination: Pagination,
    pub metadata: DataMetadata,
}

/// Entity in the response.
#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    SerdeSerialize,
    SerdeDeserialize,
    Debug,
    Clone,
    PartialEq,
)]
#[archive(check_bytes)]
pub struct Entity {
    pub id: u64,
    pub name: String,
    pub entity_type: String,
    pub score: f64,
    pub tags: Vec<String>,
    pub attributes: Vec<Attribute>,
    pub active: bool,
}

/// Entity attribute.
#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    SerdeSerialize,
    SerdeDeserialize,
    Debug,
    Clone,
    PartialEq,
)]
#[archive(check_bytes)]
pub struct Attribute {
    pub key: String,
    pub value: AttributeValue,
}

/// Attribute value variant.
#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    SerdeSerialize,
    SerdeDeserialize,
    Debug,
    Clone,
    PartialEq,
)]
#[archive(check_bytes)]
pub enum AttributeValue {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<String>),
}

/// Pagination info.
#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    SerdeSerialize,
    SerdeDeserialize,
    Debug,
    Clone,
    PartialEq,
)]
#[archive(check_bytes)]
pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub has_next: bool,
}

/// Response metadata.
#[derive(
    Archive,
    RkyvDeserialize,
    RkyvSerialize,
    SerdeSerialize,
    SerdeDeserialize,
    Debug,
    Clone,
    PartialEq,
)]
#[archive(check_bytes)]
pub struct DataMetadata {
    pub version: String,
    pub cache_ttl_seconds: u32,
    pub region: String,
}

// ─── Zero-Copy Serialization Engine ──────────────────────────────────────────

/// The zero-copy serialization engine.
/// Converts JSON ↔ rkyv binary with performance tracking.
pub struct ZeroCopyEngine {
    pub stats: SerdeStats,
}

/// Serialization/deserialization statistics.
#[derive(Debug, Clone, Default)]
pub struct SerdeStats {
    pub serialize_count: u64,
    pub deserialize_count: u64,
    pub total_serialize_ns: u64,
    pub total_deserialize_ns: u64,
    pub total_bytes_serialized: u64,
    pub total_bytes_deserialized: u64,
    pub json_parse_ns: u64,
    pub json_parse_count: u64,
}

impl SerdeStats {
    pub fn avg_serialize_ns(&self) -> f64 {
        if self.serialize_count == 0 {
            0.0
        } else {
            self.total_serialize_ns as f64 / self.serialize_count as f64
        }
    }

    pub fn avg_deserialize_ns(&self) -> f64 {
        if self.deserialize_count == 0 {
            0.0
        } else {
            self.total_deserialize_ns as f64 / self.deserialize_count as f64
        }
    }

    pub fn avg_json_parse_ns(&self) -> f64 {
        if self.json_parse_count == 0 {
            0.0
        } else {
            self.json_parse_ns as f64 / self.json_parse_count as f64
        }
    }

    /// Speedup factor of rkyv over JSON parsing.
    pub fn speedup_factor(&self) -> f64 {
        let json_avg = self.avg_json_parse_ns();
        let rkyv_avg = self.avg_deserialize_ns();
        if rkyv_avg < 1.0 {
            return 0.0;
        }
        json_avg / rkyv_avg
    }
}

impl ZeroCopyEngine {
    pub fn new() -> Self {
        Self {
            stats: SerdeStats::default(),
        }
    }

    /// Serialize an ApiResponse to rkyv binary format (zero-copy ready).
    pub fn serialize(&mut self, response: &ApiResponse) -> Vec<u8> {
        let start = Instant::now();
        let bytes = rkyv::to_bytes::<_, 4096>(response)
            .expect("rkyv serialization failed")
            .to_vec();
        let elapsed = start.elapsed().as_nanos() as u64;

        self.stats.serialize_count += 1;
        self.stats.total_serialize_ns += elapsed;
        self.stats.total_bytes_serialized += bytes.len() as u64;

        bytes
    }

    /// Zero-copy access: cast raw bytes directly to the archived struct.
    /// This is the ~1.4ns access path — no deserialization occurs.
    pub fn access_archived(bytes: &[u8]) -> Option<&ArchivedApiResponse> {
        rkyv::check_archived_root::<ApiResponse>(bytes).ok()
    }

    /// Full deserialization from rkyv binary back to owned struct.
    /// Still orders of magnitude faster than JSON parsing.
    pub fn deserialize_from(&mut self, bytes: &[u8]) -> Option<ApiResponse> {
        let start = Instant::now();
        let archived = rkyv::check_archived_root::<ApiResponse>(bytes).ok()?;
        let result: ApiResponse = archived.deserialize(&mut rkyv::Infallible).ok()?;
        let elapsed = start.elapsed().as_nanos() as u64;

        self.stats.deserialize_count += 1;
        self.stats.total_deserialize_ns += elapsed;
        self.stats.total_bytes_deserialized += bytes.len() as u64;

        Some(result)
    }

    /// Parse JSON string to ApiResponse (for comparison benchmarking).
    pub fn parse_json(&mut self, json: &str) -> Option<ApiResponse> {
        let start = Instant::now();
        let result: Result<ApiResponse, _> = serde_json::from_str(json);
        let elapsed = start.elapsed().as_nanos() as u64;

        self.stats.json_parse_count += 1;
        self.stats.json_parse_ns += elapsed;

        result.ok()
    }

    /// Serialize ApiResponse to JSON (for comparison).
    pub fn to_json(response: &ApiResponse) -> String {
        serde_json::to_string(response).unwrap_or_default()
    }

    /// Build a sample response for benchmarking.
    pub fn sample_response(entity_count: usize) -> ApiResponse {
        let entities: Vec<Entity> = (0..entity_count)
            .map(|i| Entity {
                id: i as u64,
                name: format!("entity-{}", i),
                entity_type: "record".to_string(),
                score: (i as f64) * 0.1,
                tags: vec!["alpha".to_string(), "beta".to_string()],
                attributes: vec![
                    Attribute {
                        key: "region".to_string(),
                        value: AttributeValue::String("us-east-1".to_string()),
                    },
                    Attribute {
                        key: "weight".to_string(),
                        value: AttributeValue::Number(42.5),
                    },
                ],
                active: i % 2 == 0,
            })
            .collect();

        ApiResponse {
            status_code: 200,
            request_id: "req-abc-123".to_string(),
            timestamp_ms: 1700000000000,
            headers: vec![
                HeaderPair {
                    key: "Content-Type".to_string(),
                    value: "application/octet-stream".to_string(),
                },
                HeaderPair {
                    key: "X-Request-Id".to_string(),
                    value: "req-abc-123".to_string(),
                },
            ],
            body: ResponseBody::Structured(StructuredData {
                entities,
                pagination: Pagination {
                    page: 1,
                    per_page: 50,
                    total: 1000,
                    has_next: true,
                },
                metadata: DataMetadata {
                    version: "2.0".to_string(),
                    cache_ttl_seconds: 300,
                    region: "us-east-1".to_string(),
                },
            }),
        }
    }
}

/// Print zero-copy serde performance report.
pub fn print_serde_report(stats: &SerdeStats) {
    use console::style;
    println!();
    println!(
        "  {} {}",
        style("Zero-Copy Serialization Report").cyan().bold(),
        style("━━━━━━━━━━━━━━━━━━━━━━━━").dim()
    );
    println!(
        "  {} rkyv serialize:    {:.0} ns avg ({} ops, {} bytes)",
        style("⚡").yellow(),
        stats.avg_serialize_ns(),
        stats.serialize_count,
        stats.total_bytes_serialized
    );
    println!(
        "  {} rkyv deserialize:  {:.0} ns avg ({} ops)",
        style("⚡").yellow(),
        stats.avg_deserialize_ns(),
        stats.deserialize_count
    );
    println!(
        "  {} JSON parse:        {:.0} ns avg ({} ops)",
        style("▸").dim(),
        stats.avg_json_parse_ns(),
        stats.json_parse_count
    );
    let speedup = stats.speedup_factor();
    if speedup > 0.0 {
        println!(
            "  {} Speedup:           {}x faster than JSON",
            style("🚀").yellow(),
            style(format!("{:.0}", speedup)).green().bold()
        );
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_serialize_deserialize() {
        let mut engine = ZeroCopyEngine::new();
        let response = ZeroCopyEngine::sample_response(5);

        let bytes = engine.serialize(&response);
        assert!(!bytes.is_empty());

        let restored = engine.deserialize_from(&bytes).unwrap();
        assert_eq!(restored.status_code, 200);
        assert_eq!(restored.request_id, "req-abc-123");
    }

    #[test]
    fn test_zero_copy_access() {
        let mut engine = ZeroCopyEngine::new();
        let response = ZeroCopyEngine::sample_response(3);
        let bytes = engine.serialize(&response);

        let archived = ZeroCopyEngine::access_archived(&bytes).unwrap();
        assert_eq!(archived.status_code, 200);
        assert_eq!(archived.request_id.as_str(), "req-abc-123");
    }

    #[test]
    fn test_json_roundtrip() {
        let response = ZeroCopyEngine::sample_response(2);
        let json = ZeroCopyEngine::to_json(&response);
        assert!(json.contains("entity-0"));

        let mut engine = ZeroCopyEngine::new();
        let parsed = engine.parse_json(&json).unwrap();
        assert_eq!(parsed.status_code, 200);
    }

    #[test]
    fn test_benchmark_table() {
        let table = benchmark_table();
        assert_eq!(table.len(), 4);
        assert!(table[3].supports_mutation);
        assert_eq!(table[3].framework, "rkyv (Rust Native)");
    }

    #[test]
    fn test_nested_entity_serialization() {
        let mut engine = ZeroCopyEngine::new();
        let response = ZeroCopyEngine::sample_response(100);
        let bytes = engine.serialize(&response);
        let restored = engine.deserialize_from(&bytes).unwrap();
        if let ResponseBody::Structured(data) = &restored.body {
            assert_eq!(data.entities.len(), 100);
            assert_eq!(data.pagination.total, 1000);
        } else {
            panic!("Expected Structured body");
        }
    }

    #[test]
    fn test_serde_stats() {
        let mut engine = ZeroCopyEngine::new();
        let resp = ZeroCopyEngine::sample_response(1);
        let _ = engine.serialize(&resp);
        assert_eq!(engine.stats.serialize_count, 1);
        assert!(engine.stats.total_bytes_serialized > 0);
    }
}
