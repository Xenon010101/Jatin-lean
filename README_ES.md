🌐 **Idiomas:**
[![EN](https://img.shields.io/badge/lang-English-blue)](README.md)
[![HI](https://img.shields.io/badge/lang-हिन्दी-orange)](README_HI.md)
[![ES](https://img.shields.io/badge/lang-Español-green)](README_ES.md)
[![FR](https://img.shields.io/badge/lang-Français-red)](README_FR.md)
[![ZH](https://img.shields.io/badge/lang-中文-yellow)](README_ZH.md)

---

# 🚀 jatin-lean v1.0.0 - Plataforma Universal de Optimización del Sistema

> Plataforma de optimización de nivel empresarial con **bindings nativos de Node.js** y CLI profesional. Reduce el espacio en disco hasta un **50%** aprovechando optimizaciones a nivel de hardware (io_uring, SIMD, eBPF) para un rendimiento sin igual.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![npm](https://img.shields.io/npm/v/jatin-lean.svg)](https://www.npmjs.com/package/jatin-lean)
[![Downloads](https://img.shields.io/npm/dm/jatin-lean.svg)](https://www.npmjs.com/package/jatin-lean)

---

## 🎯 Novedades en v1.0.0

### 🔥 Bindings Nativos N-API
- **Integración directa Rust ↔ JavaScript** - Sin sobrecarga de procesos adicionales
- **10-100x más rápido** que el enfoque de envoltura CLI
- **Async/await real** con promesas nativas
- **Transferencia de datos zero-copy** entre Rust y Node.js
- **Soporte completo de TypeScript** con definiciones de tipos completas

### 🎨 Interfaz CLI Profesional
- **41 comandos** organizados en 6 categorías
- **Estructura jerárquica** - `jatin-lean <categoría> <comando>`
- Soporte de **salida JSON** para todos los comandos
- Sistema de **ayuda completa** con ejemplos

---

## 📥 Instalación

```bash
npm install -g jatin-lean
```

**Requisitos:**
- Node.js >= 14
- Cadena de herramientas Rust (para compilar los bindings nativos)

---

## 🚀 Inicio Rápido

### Como Biblioteca Node.js (¡NUEVO!)

```javascript
const lean = require('jatin-lean');

async function optimize() {
  // Escanear node_modules
  const scan = await lean.scanNodeModules('.');
  console.log('Ahorro potencial:', scan.savingsPercentage.toFixed(1), '%');
  
  // Verificar salud del proyecto
  const health = await lean.checkHealth('.');
  console.log('Estado de salud:', health.overallHealth);
  
  // Evaluar rendimiento del sistema
  const system = await lean.assessSystem();
  console.log('Puntuación del sistema:', system.overallScore);
  
  // Ejecutar benchmarks
  const benchmarks = await lean.runBenchmarks();
  benchmarks.forEach(b => {
    console.log(`${b.name}: ${b.opsPerSec.toFixed(0)} ops/sec`);
  });
}

optimize();
```

### Como Herramienta CLI

```bash
# Escanear node_modules
jatin-lean node scan

# Ejecutar verificación de salud
jatin-lean node health

# Evaluación del sistema
jatin-lean system assess

# Ejecutar benchmarks
jatin-lean bench simd
```

---

## 📚 Referencia de la API Node.js

### Optimización de Módulos Node

```javascript
// Escanear oportunidades de optimización
const scan = await lean.scanNodeModules(projectPath);
// Devuelve: { totalPackages, totalSize, potentialSavings, savingsPercentage, ... }

// Verificación de salud
const health = await lean.checkHealth(projectPath);
// Devuelve: { overallHealth, missingDeps, circularDeps, outdatedCount, securityIssues }

// Encontrar archivos duplicados
const dedup = await lean.findDuplicates(projectPath);
// Devuelve: { duplicateGroups, totalDuplicates, wastedSpace, potentialSavings }

// Analizar potencial de compresión
const compressionSavings = await lean.analyzeCompression(projectPath);
// Devuelve: number (porcentaje)

// Analizar potencial de tree-shaking
const treeshakeSavings = await lean.analyzeTreeshake(projectPath);
// Devuelve: number (porcentaje)

// Obtener tamaño del grafo de dependencias
const depsCount = await lean.getDependencyGraph(projectPath);
// Devuelve: number
```

### Optimización del Sistema

```javascript
// Evaluar rendimiento del sistema
const assessment = await lean.assessSystem();
// Devuelve: { overallScore, cpuScore, memoryScore, ioScore, recommendations }

// Detectar capacidades de CPU
const cpuTier = await lean.detectCpuCapabilities();
// Devuelve: string (ej. "AVX2 (256-bit)")

// Ejecutar benchmarks
const benchmarks = await lean.runBenchmarks();
// Devuelve: [{ name, meanNs, medianNs, minNs, maxNs, opsPerSec }, ...]
```

### Funciones de Utilidad

```javascript
// Obtener versión
const version = lean.getVersion();
// Devuelve: string

// Obtener contexto compatible con IA
const context = await lean.getAiContext();
// Devuelve: { tool, version, capabilities, systemInfo }
```

---

## 🎨 Referencia de Comandos CLI

### 📦 Ecosistema Node.js (`node`)
```bash
jatin-lean node scan          # Escanear node_modules
jatin-lean node prune         # Eliminar archivos no esenciales
jatin-lean node health        # Verificación de salud
jatin-lean node dedup         # Encontrar duplicados
jatin-lean node deps          # Grafo de dependencias
jatin-lean node compress      # Análisis de compresión
jatin-lean node treeshake     # Análisis de tree-shaking
jatin-lean node audit         # Auditoría de paquetes
jatin-lean node analyze       # Análisis del proyecto
jatin-lean node watch         # Monitorear cambios
jatin-lean node policy        # Aplicar políticas
jatin-lean node visualize     # Análisis visual
jatin-lean node version       # Diagnósticos de Node/N-API
```

### 🖥️ Optimización del Sistema (`system`)
```bash
jatin-lean system assess      # Evaluación del sistema
jatin-lean system cpu         # Análisis de caché CPU
jatin-lean system memory      # Información de memoria
```

### 🛡️ Herramientas de Red (`network`)
```bash
jatin-lean network xdp        # Middleware XDP
jatin-lean network bpf        # Verificador BPF
jatin-lean network maglev     # Hash Maglev
jatin-lean network gateway    # Gateway unificado
```

### 🧠 Herramientas de Memoria (`memory`)
```bash
jatin-lean memory ipc         # Benchmarks IPC
jatin-lean memory mmap        # Mapeo de memoria
jatin-lean memory arena       # Asignador de arena
jatin-lean memory pcie        # Perfilado PCIe
```

### ⚡ Benchmarks (`bench`)
```bash
jatin-lean bench all          # Todos los benchmarks
jatin-lean bench simd         # Benchmarks SIMD
jatin-lean bench json         # Análisis JSON
jatin-lean bench io-uring     # I/O asíncrono
jatin-lean bench hash         # Hashing
```

### 📊 Análisis (`analyze`)
```bash
jatin-lean analyze all        # Análisis completo
jatin-lean analyze deps       # Dependencias
jatin-lean analyze size       # Análisis de tamaño
jatin-lean analyze cache      # Estadísticas de caché
jatin-lean analyze snapshots  # Gestión de instantáneas
```

---

## ✨ Características Principales

| Característica | Descripción | Beneficio |
|---------------|-------------|-----------|
| **⚡ Bindings Nativos** | Integración N-API con Rust | 10-100x más rápido que envolturas CLI |
| **🖥️ io_uring I/O** | I/O asíncrono zero-syscall | 10x más rápido en operaciones de archivo |
| **🧠 Optimización SIMD** | Vectorización AVX2/AVX-512 | 7x más rápido en análisis JSON |
| **🛡️ eBPF/XDP** | Red sin kernel | Procesamiento de paquetes a velocidad de línea |
| **🔗 IPC Zero-Copy** | Memoria compartida respaldada por mmap | Latencia de 102ns (490x más rápido) |
| **🗑️ Poda Inteligente** | Optimización por categorías | 30-50% de reducción en espacio de disco |

---

## 🏆 Benchmarks de Rendimiento

| Métrica | Tradicional | jatin-lean | Mejora |
|---------|-------------|------------|--------|
| **File Stat** | 120k/sec | **1.5M/sec** | **12.5x** |
| **Latencia IPC** | 50,000 ns | **102 ns** | **490x** |
| **Análisis JSON** | 450 MB/s | **3.2 GB/s** | **7x** |
| **Acceso a Memoria** | 250 ns | **1.4 ns** | **178x** |
| **Llamadas API** | 5-50ms | **<1ms** | **50x** |

---

## 🔧 Soporte TypeScript

Definiciones completas de TypeScript incluidas:

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

## 📖 Casos de Uso

### Optimización de Compilación
```javascript
// En tu script de compilación
const lean = require('jatin-lean');

async function optimizeBuild() {
  const scan = await lean.scanNodeModules('.');
  if (scan.savingsPercentage > 20) {
    console.log(`⚠️  Se puede ahorrar ${scan.savingsPercentage.toFixed(1)}% de espacio en disco`);
  }
}
```

### Integración CI/CD
```javascript
// En tu pipeline de CI
const lean = require('jatin-lean');

async function checkHealth() {
  const health = await lean.checkHealth('.');
  if (health.securityIssues > 0) {
    throw new Error(`Se encontraron ${health.securityIssues} problemas de seguridad`);
  }
}
```

### Monitoreo de Rendimiento
```javascript
// Monitorear rendimiento del sistema
const lean = require('jatin-lean');

async function monitor() {
  const system = await lean.assessSystem();
  console.log('Rendimiento del Sistema:', system.overallScore);
  
  if (system.overallScore < 70) {
    console.log('Recomendaciones:', system.recommendations);
  }
}
```

---

## 🛠️ Desarrollo

### Compilar desde el Código Fuente
```bash
# Clonar repositorio
git clone https://github.com/decodejatin/jatin-lean.git
cd jatin-lean

# Compilar bindings nativos
./build.sh

# Ejecutar pruebas
npm test

# Compilar CLI
cargo build --release
```

### Ejecutar Pruebas
```bash
# Pruebas de la API Node.js
npm test

# Pruebas de Rust
cargo test

# Pruebas de integración
cargo test --features integration
```

---

## 📊 Requisitos del Sistema

- **SO**: Linux, macOS, Windows
- **Node.js**: >= 14
- **Rust**: >= 1.70 (para compilar)
- **CPU**: x86_64 o ARM64
- **Opcional**: Soporte SIMD (AVX2/AVX-512) para máximo rendimiento

---

## 🤝 Contribuciones

¡Las contribuciones son bienvenidas! Consulta [DEVELOPER.md](DEVELOPER.md) para ver las pautas.

---

## 📄 Licencia

MIT © [Jatin Jalandhra](https://github.com/decodejatin)

---

## 🔗 Enlaces

- **GitHub**: https://github.com/decodejatin/jatin-lean
- **npm**: https://www.npmjs.com/package/jatin-lean
- **Issues**: https://github.com/decodejatin/jatin-lean/issues
- **Documentación**: [DOCUMENTATION.md](DOCUMENTATION.md)

---

**Construido con ❤️ usando Rust y N-API**