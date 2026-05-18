// Example usage of @jatin/lean in Node.js

const lean = require('@jatin/lean');

async function optimizeProject() {
  console.log('🚀 jatin-lean - Universal System Optimization Platform\n');
  
  // 1. Scan node_modules
  console.log('📦 Scanning node_modules...');
  const scanResult = await lean.scan();
  console.log(`   Found ${scanResult.totalPackages} packages`);
  console.log(`   Total size: ${(scanResult.totalSize / 1024 / 1024).toFixed(2)} MB`);
  console.log(`   Potential savings: ${(scanResult.potentialSavings / 1024 / 1024).toFixed(2)} MB (${scanResult.savingsPercentage.toFixed(1)}%)\n`);
  
  // 2. Health check
  console.log('🏥 Running health check...');
  const health = await lean.health();
  console.log(`   Overall health: ${health.overallHealth}`);
  console.log(`   Missing dependencies: ${health.missingDeps.length}`);
  console.log(`   Circular dependencies: ${health.circularDeps.length}`);
  console.log(`   Security issues: ${health.securityIssues}\n`);
  
  // 3. Find duplicates
  console.log('🔍 Finding duplicates...');
  const dedup = await lean.dedup();
  console.log(`   Duplicate groups: ${dedup.duplicateGroups}`);
  console.log(`   Wasted space: ${(dedup.wastedSpace / 1024 / 1024).toFixed(2)} MB\n`);
  
  // 4. Compression analysis
  console.log('📦 Analyzing compression...');
  const compressionSavings = await lean.compress();
  console.log(`   Gzip savings: ${compressionSavings.toFixed(1)}%\n`);
  
  // 5. Tree-shaking analysis
  console.log('🌳 Analyzing tree-shaking...');
  const treeshakeSavings = await lean.treeshake();
  console.log(`   Unused exports: ${treeshakeSavings.toFixed(1)}%\n`);
  
  // 6. System assessment
  console.log('💻 Assessing system...');
  const system = await lean.system();
  console.log(`   Overall score: ${system.overallScore}/100`);
  console.log(`   CPU score: ${system.cpuScore}/100`);
  console.log(`   Memory score: ${system.memoryScore}/100`);
  console.log(`   I/O score: ${system.ioScore}/100`);
  console.log(`   Recommendations: ${system.recommendations.length}\n`);
  
  // 7. CPU capabilities
  console.log('🔧 Detecting CPU capabilities...');
  const cpuTier = await lean.cpu();
  console.log(`   SIMD tier: ${cpuTier}\n`);
  
  // 8. Run benchmarks
  console.log('⚡ Running benchmarks...');
  const benchmarks = await lean.bench();
  console.log(`   Completed ${benchmarks.length} benchmarks:`);
  benchmarks.slice(0, 5).forEach(b => {
    const opsPerSec = b.opsPerSec >= 1000000 
      ? `${(b.opsPerSec / 1000000).toFixed(2)}M` 
      : `${(b.opsPerSec / 1000).toFixed(2)}K`;
    console.log(`   - ${b.name}: ${opsPerSec} ops/sec`);
  });
  
  console.log('\n✅ Analysis complete!');
}

// Run if called directly
if (require.main === module) {
  optimizeProject().catch(console.error);
}

module.exports = { optimizeProject };
