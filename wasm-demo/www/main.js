import init, {
    generate_membrane_primes,
    is_coprime_config,
    sieve_count_primes,
    sieve_benchmark,
    neural_predict,
    get_optimal_configs,
    get_membrane_stats
} from './pkg/prime_physics_wasm.js';

// Initialize WASM module
await init();
console.log('Prime Physics WASM module loaded');

// Tab switching
document.querySelectorAll('.tab-btn').forEach(btn => {
    btn.addEventListener('click', () => {
        const tabName = btn.dataset.tab;
        
        // Update active states
        document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
        document.querySelectorAll('.tab-content').forEach(t => t.classList.remove('active'));
        
        btn.classList.add('active');
        document.getElementById(tabName).classList.add('active');
    });
});

/* --------------------------------------------------------------------- */
/* Membrane Prime Generation Tab                                         */
/* --------------------------------------------------------------------- */

const membraneInputs = {
    base: document.getElementById('base'),
    outer: document.getElementById('outer'),
    inner: document.getElementById('inner'),
    kOuter: document.getElementById('k-outer'),
    kInner: document.getElementById('k-inner'),
    middle: document.getElementById('middle')
};

// Update value displays and check coprimality
Object.entries(membraneInputs).forEach(([key, input]) => {
    input.addEventListener('input', () => {
        const valueSpan = document.getElementById(`${input.id}-value`);
        valueSpan.textContent = input.value;
        
        if (key === 'base' || key === 'outer' || key === 'inner') {
            updateCoprimeStatus();
            updateStructurePreview();
        } else {
            updateStructurePreview();
        }
    });
});

function updateCoprimeStatus() {
    const base = parseInt(membraneInputs.base.value);
    const outer = parseInt(membraneInputs.outer.value);
    const inner = parseInt(membraneInputs.inner.value);
    
    const isCoprime = is_coprime_config(base, outer, inner);
    const status = document.getElementById('coprime-status');
    
    if (isCoprime) {
        status.textContent = '✅ Boundary digits are coprime to base';
        status.style.color = 'var(--success)';
    } else {
        status.textContent = '❌ Boundary digits must be coprime to base for optimal results';
        status.style.color = 'var(--danger)';
    }
}

function updateStructurePreview() {
    const base = parseInt(membraneInputs.base.value);
    const outer = membraneInputs.outer.value;
    const inner = membraneInputs.inner.value;
    const kOuter = parseInt(membraneInputs.kOuter.value);
    const kInner = parseInt(membraneInputs.kInner.value);
    const middle = membraneInputs.middle.value;
    
    // Build structure string
    let structure = outer;
    structure += ' 0'.repeat(kOuter);
    structure += ' ' + inner;
    structure += ' 0'.repeat(kInner);
    structure += ' ' + middle;
    structure += ' 0'.repeat(kInner);
    structure += ' ' + inner;
    structure += ' 0'.repeat(kOuter);
    structure += ' ' + outer;
    
    document.getElementById('structure-preview').textContent = 
        `${structure} (base ${base})`;
}

// Load optimal configuration
document.getElementById('optimal-config').addEventListener('click', () => {
    const base = parseInt(membraneInputs.base.value);
    const configs = get_optimal_configs(base);
    
    if (configs.length >= 4) {
        membraneInputs.outer.value = configs[0];
        membraneInputs.inner.value = configs[1];
        membraneInputs.kOuter.value = configs[2];
        membraneInputs.kInner.value = configs[3];
        
        // Update displays
        document.getElementById('outer-value').textContent = configs[0];
        document.getElementById('inner-value').textContent = configs[1];
        document.getElementById('k-outer-value').textContent = configs[2];
        document.getElementById('k-inner-value').textContent = configs[3];
        
        updateCoprimeStatus();
        updateStructurePreview();
    }
});

// Generate batch of candidates
document.getElementById('generate-batch').addEventListener('click', async () => {
    const base = parseInt(membraneInputs.base.value);
    const outer = parseInt(membraneInputs.outer.value);
    const inner = parseInt(membraneInputs.inner.value);
    const kOuter = parseInt(membraneInputs.kOuter.value);
    const kInner = parseInt(membraneInputs.kInner.value);
    const middleStart = parseInt(membraneInputs.middle.value);
    
    try {
        const results = generate_membrane_primes(
            base, outer, inner, kOuter, kInner, middleStart, 20
        );
        
        const candidatesList = document.getElementById('candidates-list');
        candidatesList.innerHTML = '';
        
        // Process results (groups of 3: low32, high32, isPrime)
        for (let i = 0; i < results.length; i += 3) {
            const valueLow = results[i];
            const valueHigh = results[i + 1];
            const isPrime = results[i + 2] === 1;
            
            let valueStr;
            if (valueLow === 0xFFFFFFFF && valueHigh === 0xFFFFFFFF) {
                valueStr = 'Number too large';
            } else {
                const value = valueLow + (valueHigh * 0x100000000);
                valueStr = value.toLocaleString();
            }
            
            const card = document.createElement('div');
            card.className = `candidate-card ${isPrime ? 'prime' : 'composite'}`;
            card.innerHTML = `
                <div class="value">${valueStr}</div>
                <div class="label">${isPrime ? 'PRIME' : 'Composite'}</div>
            `;
            candidatesList.appendChild(card);
        }
        
        // Update statistics
        try {
            const stats = get_membrane_stats(base, outer, inner, kOuter, kInner, 100);
            const statsDiv = document.getElementById('membrane-stats');
            statsDiv.innerHTML = `
                <p><strong>Success Rate:</strong> ${stats[1].toFixed(1)}% (${Math.floor(stats[0])} primes in 100 samples)</p>
                <p><strong>Average Digits:</strong> ${stats[2].toFixed(1)}</p>
            `;
        } catch (e) {
            console.error('Stats error:', e);
        }
    } catch (error) {
        alert('Error: ' + (error.message ? error.message() : error));
    }
});

/* --------------------------------------------------------------------- */
/* Prime Sieve Tab                                                       */
/* --------------------------------------------------------------------- */

let sieveChart = null;
const sieveResults = [];

document.getElementById('run-sieve').addEventListener('click', () => {
    const limit = parseInt(document.getElementById('sieve-limit').value);
    const output = document.getElementById('sieve-output');
    
    output.innerHTML = 'Running sieve...';
    
    // Use setTimeout to let UI update
    setTimeout(async () => {
        try {
            const start = performance.now();
            const count = sieve_count_primes(limit);
            const elapsed = performance.now() - start;
            
            output.innerHTML = `
                <div>Found <strong>${count.toLocaleString()}</strong> primes ≤ ${limit.toLocaleString()}</div>
                <div>Time: <strong>${elapsed.toFixed(2)} ms</strong></div>
                <div>Speed: <strong>${(limit / elapsed / 1000).toFixed(2)} million candidates/sec</strong></div>
            `;
        } catch (error) {
            output.innerHTML = `<div style="color: var(--danger)">Error: ${error.message ? error.message() : error}</div>`;
        }
    }, 10);
});

document.getElementById('benchmark-sieve').addEventListener('click', () => {
    const output = document.getElementById('sieve-output');
    output.innerHTML = 'Running benchmark suite...';
    
    setTimeout(() => {
        const limits = [10000, 100000, 1000000, 10000000];
        const results = [];
        
        limits.forEach(limit => {
            const [count, time] = sieve_benchmark(limit);
            results.push({ limit, count, time });
        });
        
        // Update output
        output.innerHTML = '<h4>Benchmark Results</h4>';
        results.forEach(r => {
            const div = document.createElement('div');
            div.style.marginBottom = '0.5rem';
            div.innerHTML = `
                <strong>${r.limit.toLocaleString()}</strong>: 
                ${r.count.toLocaleString()} primes in ${r.time.toFixed(2)} ms
                (${(r.limit / r.time / 1000).toFixed(2)} M/s)
            `;
            output.appendChild(div);
        });
        
        // Draw performance chart
        drawSieveChart(results);
    }, 10);
});

function drawSieveChart(results) {
    const canvas = document.getElementById('sieve-chart');
    const ctx = canvas.getContext('2d');
    
    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Calculate scales
    const padding = 40;
    const width = canvas.width - 2 * padding;
    const height = canvas.height - 2 * padding;
    
    const maxLimit = Math.max(...results.map(r => r.limit));
    const maxTime = Math.max(...results.map(r => r.time));
    
    // Draw axes
    ctx.strokeStyle = '#333';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(padding, padding);
    ctx.lineTo(padding, canvas.height - padding);
    ctx.lineTo(canvas.width - padding, canvas.height - padding);
    ctx.stroke();
    
    // Draw data points and lines
    ctx.strokeStyle = '#4a90e2';
    ctx.fillStyle = '#4a90e2';
    ctx.lineWidth = 3;
    ctx.beginPath();
    
    results.forEach((r, i) => {
        const x = padding + (r.limit / maxLimit) * width;
        const y = canvas.height - padding - (r.time / maxTime) * height;
        
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
        
        // Draw point
        ctx.fillRect(x - 4, y - 4, 8, 8);
    });
    ctx.stroke();
    
    // Labels
    ctx.fillStyle = '#333';
    ctx.font = '14px sans-serif';
    ctx.textAlign = 'center';
    ctx.fillText('Limit (n)', canvas.width / 2, canvas.height - 10);
    
    ctx.save();
    ctx.translate(15, canvas.height / 2);
    ctx.rotate(-Math.PI / 2);
    ctx.fillText('Time (ms)', 0, 0);
    ctx.restore();
}

/* --------------------------------------------------------------------- */
/* Neural Network Tab                                                    */
/* --------------------------------------------------------------------- */

const neuralInputs = new Int8Array(8);
const inputElements = [];

// Create input controls
const neuralInputsDiv = document.getElementById('neural-inputs');
for (let i = 0; i < 8; i++) {
    const div = document.createElement('div');
    div.className = 'neural-input';
    
    const input = document.createElement('input');
    input.type = 'number';
    input.min = -128;
    input.max = 127;
    input.value = 0;
    input.addEventListener('input', () => {
        neuralInputs[i] = parseInt(input.value) || 0;
        updateNeuralOutput();
    });
    
    div.innerHTML = `<label>x[${i}]</label>`;
    div.appendChild(input);
    neuralInputsDiv.appendChild(div);
    inputElements.push(input);
}

function updateNeuralOutput() {
    const start = performance.now();
    const output = neural_predict(neuralInputs);
    const elapsed = performance.now() - start;
    
    document.getElementById('neural-output').textContent = output;
    document.getElementById('neural-timing').textContent = 
        `${(elapsed * 1000).toFixed(1)} μs`;
}

document.getElementById('neural-random').addEventListener('click', () => {
    for (let i = 0; i < 8; i++) {
        neuralInputs[i] = Math.floor(Math.random() * 256) - 128;
        inputElements[i].value = neuralInputs[i];
    }
    updateNeuralOutput();
});

document.getElementById('neural-zero').addEventListener('click', () => {
    for (let i = 0; i < 8; i++) {
        neuralInputs[i] = 0;
        inputElements[i].value = 0;
    }
    updateNeuralOutput();
});

document.getElementById('neural-pattern').addEventListener('click', () => {
    const pattern = [1, -1, 2, -2, 4, -4, 8, -8];
    for (let i = 0; i < 8; i++) {
        neuralInputs[i] = pattern[i];
        inputElements[i].value = pattern[i];
    }
    updateNeuralOutput();
});

// Initialize
updateCoprimeStatus();
updateStructurePreview();
updateNeuralOutput();