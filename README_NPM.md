# @jatin/lean

> Universal System Optimization Platform - Node.js bindings for jatin-lean

High-performance native Node.js bindings for jatin-lean, providing comprehensive system optimization, node_modules analysis, and performance tuning capabilities.

## Features

- 🚀 **Native Performance** - Written in Rust, exposed via N-API
- 📦 **Node.js Optimization** - Scan, analyze, and optimize node_modules
- 💻 **System Tuning** - Hardware-level performance assessment
- 🔍 **Deep Analysis** - Duplicates, compression, tree-shaking
- ⚡ **SIMD Acceleration** - AVX2/AVX-512 optimized operations
- 🎯 **Zero Dependencies** - Pure native addon, no JS dependencies

## Installation

```bash
npm install @jatin/lean
```

## Quick Start

```javascript
const lean = require('@jatin/lean');

async function optimize() {
  // Scan node_modules
  const scan = await lean.scan();
  console.log(`Potential savings: ${scan.savingsPercentage}%`);
  
  // Health check
  const health = await lean.health();
  console.log(`Health: ${health.overallHealth}`);
  
  // System assessment
  const system = await lean.system();
  console.log(`System score: ${system.overallScore}/100`);
}

optimize();
```

## API Reference

### Node.js Optimization

#### `scanNodeModules(path?): Promise<ScanResult>`

Scan node_modules for optimization opportunities.

```javascript
const result = await lean.scanNodeModules('./my-project');
console.log(result);
// {
//   totalFiles: 125000,
//   totalSize: 524288000,
//   totalPackages: 1247,
//   candidatesCount: 45000,
//   potentialSavings: 209715200,
//   savingsPercentage: 40.0
// }
```

#### `checkHealth(path?): Promise<HealthResult>`

Run comprehensive health check on node_modules.

```javascript
const health = await lean.checkHealth();
console.log(health);
// {
//   missingDeps: ['react', 'lodash'],
//   circularDeps: ['a -> b -> c -> a'],
//   outdatedCount: 15,
//   securityIssues: 3,
//   overallHealth: 'warning'
// }
```

#### `findDuplicates(path?): Promise<DedupResult>`

Find duplicate files across packages.

```javascript
const dedup = await lean.findDuplicates();
console.log(`Wasted space: ${dedup.wastedSpace / 1024 / 1024} MB`);
```

#### `analyzeCompression(path?): Promise<number>`

Analyze compression potential (returns percentage).

```javascript
const savings = await lean.analyzeCompression();
console.log(`Gzip savings: ${savings}%`);
```

#### `analyzeTreeshake(path?): Promise<number>`

Analyze tree-shaking potential (returns percentage).

```javascript
const unused = await lean.analyzeTreeshake();
console.log(`Unused exports: ${unused}%`);
```

#### `getDependencyGraph(path?): Promise<number>`

Get total dependency count.

```javascript
const totalDeps = await lean.getDependencyGraph();
console.log(`Total dependencies: ${totalDeps}`);
```

### System Optimization

#### `assessSystem(): Promise<SystemAssessment>`

Assess system performance and get recommendations.

```javascript
const assessment = await lean.assessSystem();
console.log(assessment);
// {
//   recommendations: [
//     'CPU: Set governor to performance',
//     'Memory: Enable transparent huge pages',
//     'Network: Enable TCP_FASTOPEN'
//   ],
//   cpuScore: 75,
//   memoryScore: 80,
//   ioScore: 70,
//   overallScore: 75
// }
```

#### `detectCpuCapabilities(): Promise<string>`

Detect CPU SIMD capabilities.

```javascript
const simdTier = await lean.detectCpuCapabilities();
console.log(`SIMD: ${simdTier}`); // "AVX2 (256-bit)"
```

### Benchmarking

#### `runBenchmarks(): Promise<BenchmarkResult[]>`

Run comprehensive benchmark suite.

```javascript
const benchmarks = await lean.runBenchmarks();
benchmarks.forEach(b => {
  console.log(`${b.name}: ${b.opsPerSec} ops/sec`);
});
```

### Utilities

#### `getVersion(): string`

Get jatin-lean version.

```javascript
console.log(lean.getVersion()); // "2.0.0"
```

#### `getAiContext(): Promise<AiContext>`

Get AI-friendly context about the tool and system.

```javascript
const context = await lean.getAiContext();
console.log(context);
// {
//   tool: 'jatin-lean',
//   version: '2.0.0',
//   capabilities: ['node_modules_optimization', 'system_performance_tuning', ...],
//   systemInfo: {
//     os: 'linux',
//     arch: 'x86_64',
//     cpuCores: 18,
//     simdTier: 'AVX2 (256-bit)'
//   }
// }
```

## TypeScript Support

Full TypeScript definitions included:

```typescript
import * as lean from '@jatin/lean';

async function analyze(): Promise<void> {
  const result: lean.ScanResult = await lean.scan();
  const health: lean.HealthResult = await lean.health();
  const system: lean.SystemAssessment = await lean.system();
}
```

## Advanced Usage

### CI/CD Integration

```javascript
const lean = require('@jatin/lean');

async function ciCheck() {
  const scan = await lean.scan();
  
  // Fail if node_modules is too large
  if (scan.totalSize > 500 * 1024 * 1024) {
    console.error('❌ node_modules exceeds 500MB');
    process.exit(1);
  }
  
  // Fail if health is critical
  const health = await lean.health();
  if (health.overallHealth === 'critical') {
    console.error('❌ Critical health issues detected');
    process.exit(1);
  }
  
  console.log('✅ All checks passed');
}

ciCheck();
```

### Performance Monitoring

```javascript
const lean = require('@jatin/lean');

async function monitor() {
  const system = await lean.system();
  
  // Alert if system score is low
  if (system.overallScore < 60) {
    console.warn('⚠️ System performance is degraded');
    system.recommendations.forEach(r => console.log(`  - ${r}`));
  }
  
  // Log CPU capabilities
  const cpu = await lean.cpu();
  console.log(`CPU: ${cpu}`);
}

setInterval(monitor, 60000); // Every minute
```

### Build Optimization

```javascript
const lean = require('@jatin/lean');

async function optimizeBuild() {
  // Analyze before build
  const before = await lean.scan();
  console.log(`Before: ${before.totalSize / 1024 / 1024} MB`);
  
  // Find duplicates
  const dedup = await lean.dedup();
  if (dedup.duplicateGroups > 0) {
    console.log(`Found ${dedup.duplicateGroups} duplicate groups`);
    console.log(`Potential savings: ${dedup.potentialSavings / 1024 / 1024} MB`);
  }
  
  // Analyze tree-shaking
  const treeshake = await lean.treeshake();
  if (treeshake > 20) {
    console.log(`⚠️ ${treeshake}% unused exports detected`);
  }
}

optimizeBuild();
```

## Performance

jatin-lean is built with performance in mind:

- **SIMD Acceleration**: AVX2/AVX-512 for JSON parsing and data processing
- **Zero-Copy**: Efficient memory usage with zero-copy operations
- **Lock-Free**: Lock-free algorithms for concurrent operations
- **Native Speed**: Rust implementation with N-API bindings

### Benchmarks

```
SIMD JSON Parsing:    58.9 MB/s (0.47 Gbps)
Maglev Hashing:       1.88 ms for 65,537 entries
Request Coalescing:   1M+ requests/sec
Shared Memory IPC:    GB/s throughput
```

## Platform Support

- ✅ Linux (x86_64, aarch64)
- ✅ macOS (x86_64, Apple Silicon)
- ✅ Windows (x86_64)

## Requirements

- Node.js >= 14
- Rust toolchain (for building from source)

## Building from Source

```bash
# Clone repository
git clone https://github.com/decodejatin/jatin-lean.git
cd jatin-lean

# Build with Node.js bindings
cargo build --release --features napi

# Test
node test.js
```

## CLI Tool

jatin-lean also provides a powerful CLI tool:

```bash
# Install CLI globally
cargo install jatin-lean

# Use CLI
jatin-lean node scan
jatin-lean system optimize --assess
jatin-lean bench all
```

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT © Jatin Jalandhra

## Links

- [GitHub](https://github.com/decodejatin/jatin-lean)
- [Documentation](https://github.com/decodejatin/jatin-lean#readme)
- [Issues](https://github.com/decodejatin/jatin-lean/issues)

## Related

- [jatin-lean CLI](https://github.com/decodejatin/jatin-lean) - Command-line tool
- [Rust crate](https://crates.io/crates/jatin-lean) - Rust library

---

**Made with ❤️ by Jatin Jalandhra**
