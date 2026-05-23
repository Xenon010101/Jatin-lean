🌐 **语言:**
[![EN](https://img.shields.io/badge/lang-English-blue)](README.md)
[![HI](https://img.shields.io/badge/lang-हिन्दी-orange)](README_HI.md)
[![ES](https://img.shields.io/badge/lang-Español-green)](README_ES.md)
[![FR](https://img.shields.io/badge/lang-Français-red)](README_FR.md)
[![ZH](https://img.shields.io/badge/lang-中文-yellow)](README_ZH.md)

---

# 🚀 jatin-lean v1.0.0 - 通用系统优化平台

> 企业级优化平台，具备**原生 Node.js 绑定**和专业 CLI 工具。利用硬件级优化（io_uring、SIMD、eBPF）将磁盘占用减少最多 **50%**，实现无与伦比的性能表现。

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![npm](https://img.shields.io/npm/v/jatin-lean.svg)](https://www.npmjs.com/package/jatin-lean)
[![Downloads](https://img.shields.io/npm/dm/jatin-lean.svg)](https://www.npmjs.com/package/jatin-lean)

---

## 🎯 v1.0.0 新特性

### 🔥 原生 N-API 绑定
- **Rust ↔ JavaScript 直接集成** - 无额外进程开销
- 比 CLI 包装方式**快 10-100 倍**
- 基于原生 Promise 的**真正 async/await**
- Rust 与 Node.js 之间的**零拷贝数据传输**
- 附带完整类型定义的**全面 TypeScript 支持**

### 🎨 专业 CLI 界面
- 分 6 个类别组织的 **41 条命令**
- **层次化结构** - `jatin-lean <类别> <命令>`
- 所有命令均支持 **JSON 输出**
- 带示例的**全面帮助**系统

---

## 📥 安装

```bash
npm install -g jatin-lean
```

**环境要求：**
- Node.js >= 14
- Rust 工具链（用于构建原生绑定）

---

## 🚀 快速开始

### 作为 Node.js 库使用（新功能！）

```javascript
const lean = require('jatin-lean');

async function optimize() {
  // 扫描 node_modules
  const scan = await lean.scanNodeModules('.');
  console.log('潜在节省空间:', scan.savingsPercentage.toFixed(1), '%');
  
  // 检查项目健康状态
  const health = await lean.checkHealth('.');
  console.log('健康状态:', health.overallHealth);
  
  // 评估系统性能
  const system = await lean.assessSystem();
  console.log('系统评分:', system.overallScore);
  
  // 运行基准测试
  const benchmarks = await lean.runBenchmarks();
  benchmarks.forEach(b => {
    console.log(`${b.name}: ${b.opsPerSec.toFixed(0)} ops/sec`);
  });
}

optimize();
```

### 作为 CLI 工具使用

```bash
# 扫描 node_modules
jatin-lean node scan

# 运行健康检查
jatin-lean node health

# 系统评估
jatin-lean system assess

# 运行基准测试
jatin-lean bench simd
```

---

## 📚 Node.js API 参考

### Node 模块优化

```javascript
// 扫描优化机会
const scan = await lean.scanNodeModules(projectPath);
// 返回: { totalPackages, totalSize, potentialSavings, savingsPercentage, ... }

// 健康检查
const health = await lean.checkHealth(projectPath);
// 返回: { overallHealth, missingDeps, circularDeps, outdatedCount, securityIssues }

// 查找重复文件
const dedup = await lean.findDuplicates(projectPath);
// 返回: { duplicateGroups, totalDuplicates, wastedSpace, potentialSavings }

// 分析压缩潜力
const compressionSavings = await lean.analyzeCompression(projectPath);
// 返回: number（百分比）

// 分析 tree-shaking 潜力
const treeshakeSavings = await lean.analyzeTreeshake(projectPath);
// 返回: number（百分比）

// 获取依赖图大小
const depsCount = await lean.getDependencyGraph(projectPath);
// 返回: number
```

### 系统优化

```javascript
// 评估系统性能
const assessment = await lean.assessSystem();
// 返回: { overallScore, cpuScore, memoryScore, ioScore, recommendations }

// 检测 CPU 能力
const cpuTier = await lean.detectCpuCapabilities();
// 返回: string（例如 "AVX2 (256-bit)"）

// 运行基准测试
const benchmarks = await lean.runBenchmarks();
// 返回: [{ name, meanNs, medianNs, minNs, maxNs, opsPerSec }, ...]
```

### 工具函数

```javascript
// 获取版本
const version = lean.getVersion();
// 返回: string

// 获取 AI 友好的上下文信息
const context = await lean.getAiContext();
// 返回: { tool, version, capabilities, systemInfo }
```

---

## 🎨 CLI 命令参考

### 📦 Node.js 生态系统（`node`）
```bash
jatin-lean node scan          # 扫描 node_modules
jatin-lean node prune         # 删除冗余文件
jatin-lean node health        # 健康检查
jatin-lean node dedup         # 查找重复文件
jatin-lean node deps          # 依赖关系图
jatin-lean node compress      # 压缩分析
jatin-lean node treeshake     # Tree-shaking 分析
jatin-lean node audit         # 包审计
jatin-lean node analyze       # 项目分析
jatin-lean node watch         # 监控变化
jatin-lean node policy        # 执行策略
jatin-lean node visualize     # 可视化分析
jatin-lean node version       # Node/N-API 构建诊断
```

### 🖥️ 系统优化（`system`）
```bash
jatin-lean system assess      # 系统评估
jatin-lean system cpu         # CPU 缓存分析
jatin-lean system memory      # 内存信息
```

### 🛡️ 网络工具（`network`）
```bash
jatin-lean network xdp        # XDP 中间件
jatin-lean network bpf        # BPF 验证器
jatin-lean network maglev     # Maglev 哈希
jatin-lean network gateway    # 统一网关
```

### 🧠 内存工具（`memory`）
```bash
jatin-lean memory ipc         # IPC 基准测试
jatin-lean memory mmap        # 内存映射
jatin-lean memory arena       # 内存池分配器
jatin-lean memory pcie        # PCIe 分析
```

### ⚡ 基准测试（`bench`）
```bash
jatin-lean bench all          # 所有基准测试
jatin-lean bench simd         # SIMD 基准测试
jatin-lean bench json         # JSON 解析
jatin-lean bench io-uring     # 异步 I/O
jatin-lean bench hash         # 哈希运算
```

### 📊 分析（`analyze`）
```bash
jatin-lean analyze all        # 完整分析
jatin-lean analyze deps       # 依赖关系
jatin-lean analyze size       # 大小分析
jatin-lean analyze cache      # 缓存统计
jatin-lean analyze snapshots  # 快照管理
```

---

## ✨ 核心特性

| 特性 | 描述 | 优势 |
|------|------|------|
| **⚡ 原生绑定** | 与 Rust 的 N-API 集成 | 比 CLI 包装快 10-100 倍 |
| **🖥️ io_uring I/O** | 零系统调用异步 I/O | 文件操作快 10 倍 |
| **🧠 SIMD 优化** | AVX2/AVX-512 向量化 | JSON 解析快 7 倍 |
| **🛡️ eBPF/XDP** | 内核旁路网络 | 线速数据包处理 |
| **🔗 零拷贝 IPC** | mmap 支持的共享内存 | 102ns 延迟（快 490 倍）|
| **🗑️ 智能精简** | 基于类别的优化 | 节省 30-50% 磁盘空间 |

---

## 🏆 性能基准测试

| 指标 | 传统方式 | jatin-lean | 提升 |
|------|---------|------------|------|
| **文件 Stat** | 120k/秒 | **1.5M/秒** | **12.5x** |
| **IPC 延迟** | 50,000 ns | **102 ns** | **490x** |
| **JSON 解析** | 450 MB/s | **3.2 GB/s** | **7x** |
| **内存访问** | 250 ns | **1.4 ns** | **178x** |
| **API 调用** | 5-50ms | **<1ms** | **50x** |

---

## 🔧 TypeScript 支持

包含完整的 TypeScript 定义：

```typescript
import * as lean from 'jatin-lean';

interface ScanResult {
  totalPackages: number;
  totalSize: number;
  potentialSavings: number;
  savingsPercentage: number;
  candidatesCount: number;
}

const scan: ScanResult = await lean.scanNodeModules('.');
```

---

## 📖 使用场景

### 构建优化
```javascript
// 在构建脚本中
const lean = require('jatin-lean');

async function optimizeBuild() {
  const scan = await lean.scanNodeModules('.');
  if (scan.savingsPercentage > 20) {
    console.log(`⚠️  可节省 ${scan.savingsPercentage.toFixed(1)}% 的磁盘空间`);
  }
}
```

### CI/CD 集成
```javascript
// 在 CI 流水线中
const lean = require('jatin-lean');

async function checkHealth() {
  const health = await lean.checkHealth('.');
  if (health.securityIssues > 0) {
    throw new Error(`发现 ${health.securityIssues} 个安全问题`);
  }
}
```

### 性能监控
```javascript
// 监控系统性能
const lean = require('jatin-lean');

async function monitor() {
  const system = await lean.assessSystem();
  console.log('系统性能评分:', system.overallScore);
  
  if (system.overallScore < 70) {
    console.log('优化建议:', system.recommendations);
  }
}
```

---

## 🛠️ 开发

### 从源码构建
```bash
# 克隆仓库
git clone https://github.com/decodejatin/jatin-lean.git
cd jatin-lean

# 构建原生绑定
./build.sh

# 运行测试
npm test

# 构建 CLI
cargo build --release
```

### 运行测试
```bash
# Node.js API 测试
npm test

# Rust 测试
cargo test

# 集成测试
cargo test --features integration
```

---

## 📊 系统要求

- **操作系统**: Linux、macOS、Windows
- **Node.js**: >= 14
- **Rust**: >= 1.70（用于构建）
- **CPU**: x86_64 或 ARM64
- **可选**: SIMD 支持（AVX2/AVX-512）以获得最高性能

---

## 🤝 贡献

欢迎贡献！请查阅 [DEVELOPER.md](DEVELOPER.md) 了解贡献指南。

---

## 📄 许可证

MIT © [Jatin Jalandhra](https://github.com/decodejatin)

---

## 🔗 链接

- **GitHub**: https://github.com/decodejatin/jatin-lean
- **npm**: https://www.npmjs.com/package/jatin-lean
- **Issues**: https://github.com/decodejatin/jatin-lean/issues
- **文档**: [DOCUMENTATION.md](DOCUMENTATION.md)

---

**用 Rust 和 N-API 以 ❤️ 构建**