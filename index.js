// Node.js bindings for jatin-lean
const { platform, arch } = require('os');
const path = require('path');

// Load the native addon
let nativeBinding;
try {
  // Try loading from the root directory first (for npm package)
  nativeBinding = require(path.join(__dirname, 'index.node'));
} catch (e1) {
  try {
    // Fallback to target/release for development
    const bindingPath = path.join(__dirname, 'target/release');
    let libName;
    
    if (platform() === 'win32') {
      libName = 'jatin_lean.dll';
    } else if (platform() === 'darwin') {
      libName = 'libjatin_lean.dylib';
    } else {
      libName = 'libjatin_lean.so';
    }
    
    nativeBinding = require(path.join(bindingPath, libName));
  } catch (e2) {
    throw new Error(
      `Failed to load jatin-lean native binding. ` +
      `Platform: ${platform()}, Arch: ${arch()}. ` +
      `Errors: ${e1.message}, ${e2.message}`
    );
  }
}

/**
 * Scan node_modules directory for optimization opportunities
 * @param {string} path - Path to project directory
 * @returns {Promise<ScanResult>}
 */
async function scanNodeModules(projectPath = '.') {
  return nativeBinding.scanNodeModules(projectPath);
}

/**
 * Run health check on node_modules
 * @param {string} path - Path to project directory
 * @returns {Promise<HealthResult>}
 */
async function checkHealth(projectPath = '.') {
  return nativeBinding.checkHealth(projectPath);
}

/**
 * Find duplicate files in node_modules
 * @param {string} path - Path to project directory
 * @returns {Promise<DedupResult>}
 */
async function findDuplicates(projectPath = '.') {
  return nativeBinding.findDuplicates(projectPath);
}

/**
 * Analyze compression potential
 * @param {string} path - Path to project directory
 * @returns {Promise<number>} Compression savings percentage
 */
async function analyzeCompression(projectPath = '.') {
  return nativeBinding.analyzeCompression(projectPath);
}

/**
 * Analyze tree-shaking potential
 * @param {string} path - Path to project directory
 * @returns {Promise<number>} Tree-shaking savings percentage
 */
async function analyzeTreeshake(projectPath = '.') {
  return nativeBinding.analyzeTreeshake(projectPath);
}

/**
 * Get dependency graph size
 * @param {string} path - Path to project directory
 * @returns {Promise<number>} Total dependencies count
 */
async function getDependencyGraph(projectPath = '.') {
  return nativeBinding.getDependencyGraph(projectPath);
}

/**
 * Assess system performance
 * @returns {Promise<SystemAssessment>}
 */
async function assessSystem() {
  return nativeBinding.assessSystem();
}

/**
 * Detect CPU capabilities
 * @returns {Promise<string>} SIMD tier (e.g., "AVX2")
 */
async function detectCpuCapabilities() {
  return nativeBinding.detectCpuCapabilities();
}

/**
 * Run benchmark suite
 * @returns {Promise<BenchmarkResult[]>}
 */
async function runBenchmarks() {
  return nativeBinding.runBenchmarks();
}

/**
 * Get tool version
 * @returns {string}
 */
function getVersion() {
  return nativeBinding.getVersion();
}

/**
 * Get AI-friendly context
 * @returns {Promise<AiContext>}
 */
async function getAiContext() {
  return nativeBinding.getAiContext();
}

// Export all functions
module.exports = {
  scanNodeModules,
  checkHealth,
  findDuplicates,
  analyzeCompression,
  analyzeTreeshake,
  getDependencyGraph,
  assessSystem,
  detectCpuCapabilities,
  runBenchmarks,
  getVersion,
  getAiContext,
  
  // Aliases
  scan: scanNodeModules,
  health: checkHealth,
  dedup: findDuplicates,
  compress: analyzeCompression,
  treeshake: analyzeTreeshake,
  deps: getDependencyGraph,
  system: assessSystem,
  cpu: detectCpuCapabilities,
  bench: runBenchmarks,
  version: getVersion,
  ai: getAiContext,
};
