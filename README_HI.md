🌐 **भाषाएँ:**
[![EN](https://img.shields.io/badge/lang-English-blue)](README.md)
[![HI](https://img.shields.io/badge/lang-हिन्दी-orange)](README_HI.md)
[![ES](https://img.shields.io/badge/lang-Español-green)](README_ES.md)
[![FR](https://img.shields.io/badge/lang-Français-red)](README_FR.md)
[![ZH](https://img.shields.io/badge/lang-中文-yellow)](README_ZH.md)

---

# 🚀 jatin-lean v1.0.0 - सार्वभौमिक सिस्टम ऑप्टिमाइज़ेशन प्लेटफ़ॉर्म

> **नेटिव Node.js बाइंडिंग** और प्रोफ़ेशनल CLI के साथ एंटरप्राइज़-ग्रेड ऑप्टिमाइज़ेशन प्लेटफ़ॉर्म। हार्डवेयर-स्तरीय ऑप्टिमाइज़ेशन (io_uring, SIMD, eBPF) का उपयोग करते हुए डिस्क फ़ुटप्रिंट को **50% तक** कम करें और बेजोड़ प्रदर्शन पाएं।

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![npm](https://img.shields.io/npm/v/jatin-lean.svg)](https://www.npmjs.com/package/jatin-lean)
[![Downloads](https://img.shields.io/npm/dm/jatin-lean.svg)](https://www.npmjs.com/package/jatin-lean)

---

## 🎯 v1.0.0 में क्या नया है

### 🔥 नेटिव N-API बाइंडिंग

- **सीधा Rust ↔ JavaScript एकीकरण** - कोई प्रोसेस स्पॉनिंग ओवरहेड नहीं

- CLI रैपर दृष्टिकोण से **10-100 गुना तेज़**

- नेटिव प्रॉमिस के साथ **वास्तविक async/await**

- Rust और Node.js के बीच **ज़ीरो-कॉपी डेटा ट्रांसफर**

- पूर्ण टाइप डेफ़िनिशन के साथ **पूर्ण TypeScript समर्थन**

### 🎨 प्रोफ़ेशनल CLI इंटरफ़ेस

- 6 श्रेणियों में व्यवस्थित **41 कमांड**

- **श्रेणीबद्ध संरचना** - `jatin-lean <category> <command>`

- सभी कमांड के लिए **JSON आउटपुट** समर्थन

- उदाहरणों के साथ **व्यापक सहायता** प्रणाली

---

## 📥 इंस्टॉलेशन

```bash
npm install -g jatin-lean
```

**आवश्यकताएँ:**

- Node.js >= 14
- Rust टूलचेन (नेटिव बाइंडिंग बनाने के लिए)

---

## 🚀 त्वरित शुरुआत

### Node.js लाइब्रेरी के रूप में (नया!)

```javascript
const lean = require('jatin-lean');

async function optimize() {
  // node_modules स्कैन करें
  const scan = await lean.scanNodeModules('.');
  console.log('संभावित बचत:', scan.savingsPercentage.toFixed(1), '%');
  
  // प्रोजेक्ट स्वास्थ्य जाँचें
  const health = await lean.checkHealth('.');
  console.log('स्वास्थ्य स्थिति:', health.overallHealth);
  
  // सिस्टम प्रदर्शन का आकलन करें
  const system = await lean.assessSystem();
  console.log('सिस्टम स्कोर:', system.overallScore);
  
  // बेंचमार्क चलाएं
  const benchmarks = await lean.runBenchmarks();
  benchmarks.forEach(b => {
    console.log(`${b.name}: ${b.opsPerSec.toFixed(0)} ops/sec`);
  });
}

optimize();
```

### CLI टूल के रूप में

```bash
# node_modules स्कैन करें
jatin-lean node scan

# स्वास्थ्य जाँच चलाएं
jatin-lean node health

# सिस्टम आकलन
jatin-lean system assess

# बेंचमार्क चलाएं
jatin-lean bench simd
```

---

## 📚 Node.js API संदर्भ

### नोड मॉड्यूल ऑप्टिमाइज़ेशन

```javascript
// ऑप्टिमाइज़ेशन अवसरों के लिए स्कैन करें
const scan = await lean.scanNodeModules(projectPath);
// रिटर्न: { totalPackages, totalSize, potentialSavings, savingsPercentage, ... }

// स्वास्थ्य जाँच
const health = await lean.checkHealth(projectPath);
// रिटर्न: { overallHealth, missingDeps, circularDeps, outdatedCount, securityIssues }

// डुप्लिकेट फ़ाइलें खोजें
const dedup = await lean.findDuplicates(projectPath);
// रिटर्न: { duplicateGroups, totalDuplicates, wastedSpace, potentialSavings }

// संपीड़न क्षमता का विश्लेषण करें
const compressionSavings = await lean.analyzeCompression(projectPath);
// रिटर्न: number (प्रतिशत)

// ट्री-शेकिंग क्षमता का विश्लेषण करें
const treeshakeSavings = await lean.analyzeTreeshake(projectPath);
// रिटर्न: number (प्रतिशत)

// डिपेंडेंसी ग्राफ़ का आकार प्राप्त करें
const depsCount = await lean.getDependencyGraph(projectPath);
// रिटर्न: number
```

### सिस्टम ऑप्टिमाइज़ेशन

```javascript
// सिस्टम प्रदर्शन का आकलन करें
const assessment = await lean.assessSystem();
// रिटर्न: { overallScore, cpuScore, memoryScore, ioScore, recommendations }

// CPU क्षमताओं का पता लगाएं
const cpuTier = await lean.detectCpuCapabilities();
// रिटर्न: string (जैसे "AVX2 (256-bit)")

// बेंचमार्क चलाएं
const benchmarks = await lean.runBenchmarks();
// रिटर्न: [{ name, meanNs, medianNs, minNs, maxNs, opsPerSec }, ...]
```

### उपयोगिता फ़ंक्शन

```javascript
// संस्करण प्राप्त करें
const version = lean.getVersion();
// रिटर्न: string

// AI-अनुकूल संदर्भ प्राप्त करें
const context = await lean.getAiContext();
// रिटर्न: { tool, version, capabilities, systemInfo }
```

---

## 🎨 CLI कमांड संदर्भ

### 📦 Node.js इकोसिस्टम (`node`)
```bash
jatin-lean node scan          # node_modules स्कैन करें
jatin-lean node prune         # गैर-आवश्यक फ़ाइलें हटाएं
jatin-lean node health        # स्वास्थ्य जाँच
jatin-lean node dedup         # डुप्लिकेट खोजें
jatin-lean node deps          # डिपेंडेंसी ग्राफ़
jatin-lean node compress      # संपीड़न विश्लेषण
jatin-lean node treeshake     # ट्री-शेकिंग विश्लेषण
jatin-lean node audit         # पैकेज ऑडिट
jatin-lean node analyze       # प्रोजेक्ट विश्लेषण
jatin-lean node watch         # परिवर्तनों की निगरानी
jatin-lean node policy        # नीतियाँ लागू करें
jatin-lean node visualize     # विज़ुअल विश्लेषण
jatin-lean node version       # Node/N-API बिल्ड डायग्नोस्टिक्स
```

### 🖥️ सिस्टम ऑप्टिमाइज़ेशन (`system`)
```bash
jatin-lean system assess      # सिस्टम आकलन
jatin-lean system cpu         # CPU कैश विश्लेषण
jatin-lean system memory      # मेमोरी जानकारी
```

### 🛡️ नेटवर्क टूल (`network`)
```bash
jatin-lean network xdp        # XDP मिडलवेयर
jatin-lean network bpf        # BPF वेरिफायर
jatin-lean network maglev     # Maglev हैशिंग
jatin-lean network gateway    # यूनिफ़ाइड गेटवे
```

### 🧠 मेमोरी टूल (`memory`)
```bash
jatin-lean memory ipc         # IPC बेंचमार्क
jatin-lean memory mmap        # मेमोरी मैपिंग
jatin-lean memory arena       # एरीना एलोकेटर
jatin-lean memory pcie        # PCIe प्रोफ़ाइलिंग
```

### ⚡ बेंचमार्क (`bench`)
```bash
jatin-lean bench all          # सभी बेंचमार्क
jatin-lean bench simd         # SIMD बेंचमार्क
jatin-lean bench json         # JSON पार्सिंग
jatin-lean bench io-uring     # एसिंक I/O
jatin-lean bench hash         # हैशिंग
```

### 📊 विश्लेषण (`analyze`)
```bash
jatin-lean analyze all        # पूर्ण विश्लेषण
jatin-lean analyze deps       # डिपेंडेंसी
jatin-lean analyze size       # आकार विश्लेषण
jatin-lean analyze cache      # कैश आँकड़े
jatin-lean analyze snapshots  # स्नैपशॉट प्रबंधन
```

---

## ✨ मुख्य विशेषताएँ

| विशेषता | विवरण | लाभ |
|---------|-------|-----|
| **⚡ नेटिव बाइंडिंग** | Rust के साथ N-API एकीकरण | CLI रैपर से 10-100x तेज़ |
| **🖥️ io_uring I/O** | ज़ीरो-सिस्टमकॉल एसिंक I/O | 10x तेज़ फ़ाइल ऑपरेशन |
| **🧠 SIMD ऑप्टिमाइज़ेशन** | AVX2/AVX-512 वेक्टराइज़ेशन | 7x तेज़ JSON पार्सिंग |
| **🛡️ eBPF/XDP** | कर्नेल-बाईपास नेटवर्किंग | लाइन-रेट पैकेट प्रोसेसिंग |
| **🔗 ज़ीरो-कॉपी IPC** | mmap-आधारित शेयर्ड मेमोरी | 102ns लेटेंसी (490x तेज़) |
| **🗑️ स्मार्ट प्रूनिंग** | श्रेणी-आधारित ऑप्टिमाइज़ेशन | 30-50% डिस्क स्पेस बचत |

---

## 🏆 प्रदर्शन बेंचमार्क

| मेट्रिक | पारंपरिक | jatin-lean | सुधार |
|--------|----------|------------|-------|
| **File Stat** | 120k/sec | **1.5M/sec** | **12.5x** |
| **IPC लेटेंसी** | 50,000 ns | **102 ns** | **490x** |
| **JSON पार्सिंग** | 450 MB/s | **3.2 GB/s** | **7x** |
| **मेमोरी एक्सेस** | 250 ns | **1.4 ns** | **178x** |
| **API कॉल** | 5-50ms | **<1ms** | **50x** |

---

## 🔧 TypeScript समर्थन

पूर्ण TypeScript डेफ़िनिशन शामिल हैं:

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

## 📖 उपयोग के मामले

### बिल्ड ऑप्टिमाइज़ेशन
```javascript
// अपनी बिल्ड स्क्रिप्ट में
const lean = require('jatin-lean');

async function optimizeBuild() {
  const scan = await lean.scanNodeModules('.');
  if (scan.savingsPercentage > 20) {
    console.log(`⚠️  ${scan.savingsPercentage.toFixed(1)}% डिस्क स्पेस बचाई जा सकती है`);
  }
}
```

### CI/CD एकीकरण
```javascript
// CI पाइपलाइन में
const lean = require('jatin-lean');

async function checkHealth() {
  const health = await lean.checkHealth('.');
  if (health.securityIssues > 0) {
    throw new Error(`${health.securityIssues} सुरक्षा समस्याएँ मिलीं`);
  }
}
```

### प्रदर्शन निगरानी
```javascript
// सिस्टम प्रदर्शन की निगरानी करें
const lean = require('jatin-lean');

async function monitor() {
  const system = await lean.assessSystem();
  console.log('सिस्टम प्रदर्शन:', system.overallScore);
  
  if (system.overallScore < 70) {
    console.log('सुझाव:', system.recommendations);
  }
}
```

---

## 🛠️ विकास

### सोर्स से बिल्ड करें
```bash
# रिपॉजिटरी क्लोन करें
git clone https://github.com/decodejatin/jatin-lean.git
cd jatin-lean

# नेटिव बाइंडिंग बिल्ड करें
./build.sh

# परीक्षण चलाएं
npm test

# CLI बिल्ड करें
cargo build --release
```

### परीक्षण चलाएं
```bash
# Node.js API परीक्षण
npm test

# Rust परीक्षण
cargo test

# एकीकरण परीक्षण
cargo test --features integration
```

---

## 📊 सिस्टम आवश्यकताएँ

- **OS**: Linux, macOS, Windows

- **Node.js**: >= 14

- **Rust**: >= 1.70 (बिल्डिंग के लिए)

- **CPU**: x86_64 या ARM64

- **वैकल्पिक**: अधिकतम प्रदर्शन के लिए SIMD समर्थन (AVX2/AVX-512)

---

## 🤝 योगदान

योगदान स्वागत योग्य है! दिशानिर्देशों के लिए [DEVELOPER.md](DEVELOPER.md) देखें।

---

## 📄 लाइसेंस

MIT © [Jatin Jalandhra](https://github.com/decodejatin)

---

## 🔗 लिंक

- **GitHub**: https://github.com/decodejatin/jatin-lean

- **npm**: https://www.npmjs.com/package/jatin-lean

- **Issues**: https://github.com/decodejatin/jatin-lean/issues

- **दस्तावेज़ीकरण**: [DOCUMENTATION.md](DOCUMENTATION.md)

---

**Rust और N-API का उपयोग करके ❤️ के साथ बनाया गया**