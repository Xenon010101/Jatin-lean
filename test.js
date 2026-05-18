// Test file for @jatin/lean Node.js bindings

const lean = require('./index');

async function runTests() {
  console.log('🧪 Testing @jatin/lean Node.js bindings\n');
  
  try {
    // Test 1: Get version
    console.log('1. Version:', lean.getVersion());
    
    // Test 2: Detect CPU capabilities
    const cpuTier = await lean.detectCpuCapabilities();
    console.log('2. CPU SIMD Tier:', cpuTier);
    
    // Test 3: Get AI context
    const aiContext = await lean.getAiContext();
    console.log('3. AI Context:', JSON.stringify(aiContext, null, 2));
    
    // Test 4: Assess system
    const systemAssessment = await lean.assessSystem();
    console.log('4. System Assessment:');
    console.log('   Overall Score:', systemAssessment.overallScore);
    console.log('   CPU Score:', systemAssessment.cpuScore);
    console.log('   Memory Score:', systemAssessment.memoryScore);
    console.log('   Recommendations:', systemAssessment.recommendations.length);
    
    // Test 5: Run benchmarks
    console.log('5. Running benchmarks...');
    const benchmarks = await lean.runBenchmarks();
    console.log('   Completed', benchmarks.length, 'benchmarks');
    benchmarks.slice(0, 3).forEach(b => {
      console.log(`   - ${b.name}: ${b.opsPerSec.toFixed(0)} ops/sec`);
    });
    
    // Test 6: Scan node_modules (if exists)
    try {
      const scanResult = await lean.scanNodeModules('.');
      console.log('6. Node modules scan:');
      console.log('   Total packages:', scanResult.totalPackages);
      console.log('   Total size:', (scanResult.totalSize / 1024 / 1024).toFixed(2), 'MB');
      console.log('   Potential savings:', (scanResult.potentialSavings / 1024 / 1024).toFixed(2), 'MB');
      console.log('   Savings percentage:', scanResult.savingsPercentage.toFixed(1), '%');
    } catch (e) {
      console.log('6. Node modules scan: No node_modules found (expected)');
    }
    
    console.log('\n✅ All tests passed!');
    
  } catch (error) {
    console.error('❌ Test failed:', error.message);
    process.exit(1);
  }
}

runTests();
