// Lagrange TUI Web Implementation
// Mimics the terminal UI for prime exploration

class LagrangeTUI {
    constructor() {
        this.state = {
            particle1: null,
            particle2: null,
            config: { base: 10, outer: 3, inner: 3, k_outer: 1, k_inner: 1 },
            lagrangePoints: [],
            selectedPrime: 0,
            showHelp: false,
            isGenerating: false,
            statusMessage: "Press 'g' to generate prime pair",
            totalGenerations: 0,
            primesFound: 0,
            currentPrimeDistance: null,
            configIndex: 0
        };
        
        this.configs = [
            { base: 10, outer: 3, inner: 3, k_outer: 1, k_inner: 1, name: "(3,3) k=(1,1) base 10" },
            { base: 10, outer: 3, inner: 7, k_outer: 1, k_inner: 1, name: "(3,7) k=(1,1) base 10 - Exclusive!" },
            { base: 10, outer: 7, inner: 7, k_outer: 1, k_inner: 1, name: "(7,7) k=(1,1) base 10" },
            { base: 10, outer: 3, inner: 3, k_outer: 0, k_inner: 1, name: "(3,3) k=(0,1) base 10 - Breathing" },
            { base: 6, outer: 1, inner: 5, k_outer: 0, k_inner: 0, name: "(1,5) k=(0,0) base 6 - Champion" },
        ];
        
        // Some pre-calculated membrane primes
        this.knownPrimes = [
            { value: 303050303n, config: 0, seed: "5" },
            { value: 307050703n, config: 1, seed: "5" },
            { value: 30301303n, config: 0, seed: "01" },  // 2-digit middle
            { value: 707030707n, config: 2, seed: "3" }
        ];
        
        this.terminal = document.getElementById('terminal');
        this.helpOverlay = document.getElementById('help');
        
        this.setupEventListeners();
        this.render();
    }
    
    setupEventListeners() {
        document.addEventListener('keydown', (e) => {
            if (this.state.showHelp) {
                this.state.showHelp = false;
                this.helpOverlay.style.display = 'none';
                this.render();
                return;
            }
            
            switch(e.key) {
                case 'g':
                    this.generatePrimePair();
                    break;
                case 't':
                    this.testLagrangePoints();
                    break;
                case 'c':
                    this.cycleConfiguration();
                    break;
                case 'h':
                case '?':
                    this.toggleHelp();
                    break;
                case 'ArrowLeft':
                case 'ArrowRight':
                    this.state.selectedPrime = 1 - this.state.selectedPrime;
                    this.render();
                    break;
                case 'q':
                    if (confirm('Quit the TUI?')) {
                        location.reload();
                    }
                    break;
            }
        });
    }
    
    generatePrimePair() {
        this.state.isGenerating = true;
        this.state.statusMessage = "Generating prime pair...";
        this.render();
        
        // Simulate generation with timeout
        setTimeout(() => {
            // Pick two different primes
            const availablePrimes = this.knownPrimes.filter(p => 
                p.config === this.state.configIndex
            );
            
            if (availablePrimes.length >= 2) {
                this.state.particle1 = {
                    value: availablePrimes[0].value,
                    structure: this.formatStructure(availablePrimes[0].value, availablePrimes[0].seed),
                    mass: 23.21,
                    base: this.state.config.base
                };
                this.state.particle2 = {
                    value: availablePrimes[1].value,
                    structure: this.formatStructure(availablePrimes[1].value, availablePrimes[1].seed),
                    mass: 23.20,
                    base: this.state.config.base
                };
            } else {
                // Use hardcoded fallback
                this.state.particle1 = {
                    value: 303050303n,
                    structure: "3-0-[5]-0-3",
                    mass: 23.21,
                    base: 10
                };
                this.state.particle2 = {
                    value: 30301303n,
                    structure: "3-0-[01]-0-3",
                    mass: 23.20,
                    base: 10
                };
            }
            
            const distance = this.state.particle2.value - this.state.particle1.value;
            this.state.currentPrimeDistance = distance > 0n ? distance : -distance;
            
            this.calculateLagrangePoints();
            
            this.state.isGenerating = false;
            this.state.totalGenerations++;
            this.state.primesFound += 2;
            this.state.statusMessage = `Generated! Distance: ${this.state.currentPrimeDistance} | Press 't' to test L-points`;
            
            // Flash effect
            this.terminal.classList.add('flash-success');
            setTimeout(() => this.terminal.classList.remove('flash-success'), 500);
            
            this.render();
        }, 500);
    }
    
    calculateLagrangePoints() {
        if (!this.state.particle1 || !this.state.particle2) return;
        
        const p1 = this.state.particle1.value;
        const p2 = this.state.particle2.value;
        const midpoint = (p1 + p2) / 2n;
        
        this.state.lagrangePoints = [{
            type: 'L1',
            value: midpoint,
            position: [0.0, 0.0],
            fieldStrength: 1.0,
            stability: 0.5,
            isPrime: this.isPrime(midpoint)
        }];
    }
    
    testLagrangePoints() {
        if (!this.state.lagrangePoints.length) {
            this.state.statusMessage = "Generate primes first (press 'g')";
            this.render();
            return;
        }
        
        // Test each Lagrange point
        this.state.lagrangePoints.forEach(point => {
            point.tested = true;
            point.isPrime = this.isPrime(point.value);
        });
        
        this.state.statusMessage = "Lagrange points tested";
        this.render();
    }
    
    cycleConfiguration() {
        this.state.configIndex = (this.state.configIndex + 1) % this.configs.length;
        this.state.config = this.configs[this.state.configIndex];
        this.state.particle1 = null;
        this.state.particle2 = null;
        this.state.lagrangePoints = [];
        this.state.statusMessage = `Switched to config: ${this.configs[this.state.configIndex].name}`;
        this.render();
    }
    
    toggleHelp() {
        this.state.showHelp = !this.state.showHelp;
        this.helpOverlay.style.display = this.state.showHelp ? 'block' : 'none';
        if (!this.state.showHelp) {
            this.render();
        }
    }
    
    formatStructure(value, seed) {
        const str = value.toString();
        const config = this.state.config;
        
        // Simple pattern matching for visualization
        if (seed.length === 1) {
            return `${config.outer}-0-${config.inner}-0-[${seed}]-0-${config.inner}-0-${config.outer}`;
        } else {
            return `${config.outer}-0-${config.inner}-[${seed}]-${config.inner}-0-${config.outer}`;
        }
    }
    
    formatMembraneVisual(value) {
        const str = value.toString();
        const chars = str.split('');
        
        // Replace zeros with circles for visibility
        return chars.map((c, i) => {
            if (c === '0' && i > 0 && i < chars.length - 1) {
                return '◯';
            }
            return c;
        }).join('─');
    }
    
    isPrime(n) {
        // Simple primality test for demo
        if (n <= 1n) return false;
        if (n <= 3n) return true;
        if (n % 2n === 0n || n % 3n === 0n) return false;
        
        let i = 5n;
        while (i * i <= n) {
            if (n % i === 0n || n % (i + 2n) === 0n) return false;
            i += 6n;
        }
        return true;
    }
    
    render() {
        const screen = this.buildScreen();
        this.terminal.textContent = screen;
    }
    
    buildScreen() {
        const width = 150;
        const height = 40;
        
        let screen = '';
        
        // Header
        screen += '┌' + '─'.repeat(width - 2) + '┐\n';
        screen += '│' + this.centerText('⚛️  Lagrange Point Explorer - Prime Atomic Interactions', width - 2) + '│\n';
        screen += '└' + '─'.repeat(width - 2) + '┘\n';
        
        // Membrane Field
        screen += '┌⚛️ Membrane Field' + '─'.repeat(width - 19) + '┐\n';
        screen += '│' + this.centerText('╔═══════════════════════════════════════════════════════════════╗', width - 2) + '│\n';
        
        if (this.state.particle1 && this.state.particle2) {
            const p1Visual = this.formatMembraneVisual(this.state.particle1.value);
            const p2Visual = this.formatMembraneVisual(this.state.particle2.value);
            
            screen += '│' + this.centerText(`║ P₁: ${p1Visual} ║`, width - 2) + '│\n';
            screen += '│' + this.centerText(`║ P₂: ${p2Visual} ║`, width - 2) + '│\n';
            screen += '│' + this.centerText('╚═══════════════════════════════════════════════════════════════╝', width - 2) + '│\n';
            
            if (this.state.currentPrimeDistance) {
                const distBar = this.buildDistanceBar(this.state.currentPrimeDistance);
                screen += '│' + this.centerText(`║ P₁ ← ${this.state.currentPrimeDistance} → P₂ | Distance bar: ${distBar} ║`, width - 2) + '│\n';
            }
        } else {
            screen += '│' + this.centerText('║        Generate primes to see membrane interaction            ║', width - 2) + '│\n';
            screen += '│' + this.centerText('║                  Press \'g\' to begin                           ║', width - 2) + '│\n';
            screen += '│' + this.centerText('╚═══════════════════════════════════════════════════════════════╝', width - 2) + '│\n';
        }
        
        screen += '└' + '─'.repeat(width - 2) + '┘\n';
        
        // Main content area (3 columns)
        const contentHeight = 20;
        const colWidth = Math.floor((width - 4) / 3);
        
        for (let row = 0; row < contentHeight; row++) {
            if (row === 0) {
                screen += '┌⚛️  Atom 1' + '─'.repeat(colWidth - 11) + '┐';
                screen += '┌🌌 Field' + '─'.repeat(colWidth - 9) + '┐';
                screen += '┌⚛️  Atom 2' + '─'.repeat(colWidth - 11) + '┐\n';
            } else if (row === contentHeight - 1) {
                screen += '└' + '─'.repeat(colWidth - 1) + '┘';
                screen += '└' + '─'.repeat(colWidth - 1) + '┘';
                screen += '└' + '─'.repeat(colWidth - 1) + '┘\n';
            } else {
                // Content
                const atom1Content = this.getAtom1Content(row - 1);
                const fieldContent = this.getFieldContent(row - 1);
                const atom2Content = this.getAtom2Content(row - 1);
                
                screen += '│' + this.padRight(atom1Content, colWidth - 1) + '│';
                screen += '│' + this.padRight(fieldContent, colWidth - 1) + '│';
                screen += '│' + this.padRight(atom2Content, colWidth - 1) + '│\n';
            }
        }
        
        // Status bar
        const configStr = `(${this.state.config.outer},${this.state.config.inner}) k=(${this.state.config.k_outer},${this.state.config.k_inner}) b${this.state.config.base}`;
        screen += '┌' + '─'.repeat(width - 2) + '┐\n';
        screen += '│[' + configStr + '] ' + this.state.statusMessage;
        screen += ' | t:test c:config h:help q:quit';
        screen = screen.padEnd(screen.length + width - 2 - (screen.length - screen.lastIndexOf('\n') - 1)) + '│\n';
        screen += '└' + '─'.repeat(width - 2) + '┘\n';
        
        return screen;
    }
    
    buildDistanceBar(distance) {
        const barWidth = 30;
        const maxDist = 10000n;
        const fillAmount = Number(distance * BigInt(barWidth) / maxDist);
        const filled = Math.min(Math.max(1, fillAmount), barWidth);
        const empty = barWidth - filled;
        return '[' + '█'.repeat(filled) + '░'.repeat(empty) + ']';
    }
    
    getAtom1Content(row) {
        if (!this.state.particle1) return '';
        
        const lines = [
            this.centerText('Prime 1', 0),
            '',
            `Value: ${this.state.particle1.value}`,
            `Structure: ${this.state.particle1.structure}`,
            `Mass: ${this.state.particle1.mass}`,
            `Base: ${this.state.particle1.base}`
        ];
        
        return lines[row] || '';
    }
    
    getFieldContent(row) {
        const lines = ['╔═══ Lagrange Analysis ═══╗'];
        
        if (this.state.particle1 && this.state.particle2) {
            lines.push('');
            lines.push(`Range: ${this.state.particle1.value} ↔ ${this.state.particle2.value}`);
            
            if (this.state.lagrangePoints.length > 0) {
                const l1 = this.state.lagrangePoints[0];
                const primeIndicator = l1.isPrime ? ' ✓ PRIME!' : '';
                lines.push(`L₁ (midpoint): ${l1.value}${primeIndicator}`);
                lines.push('');
                lines.push(`L1: (${l1.position[0].toFixed(1)}, ${l1.position[1].toFixed(1)})`);
                lines.push(`Field: ${l1.fieldStrength.toFixed(2)} | Stability: ${l1.stability.toFixed(2)}`);
            }
        }
        
        return this.centerText(lines[row] || '', 0);
    }
    
    getAtom2Content(row) {
        if (!this.state.particle2) return '';
        
        const lines = [
            this.centerText('Prime 2', 0),
            '',
            `Value: ${this.state.particle2.value}`,
            `Structure: ${this.state.particle2.structure}`,
            `Mass: ${this.state.particle2.mass}`,
            `Base: ${this.state.particle2.base}`
        ];
        
        return lines[row] || '';
    }
    
    centerText(text, width) {
        if (width === 0) return text;
        const padding = Math.max(0, width - this.textLength(text));
        const leftPad = Math.floor(padding / 2);
        const rightPad = padding - leftPad;
        return ' '.repeat(leftPad) + text + ' '.repeat(rightPad);
    }
    
    padRight(text, width) {
        const len = this.textLength(text);
        return text + ' '.repeat(Math.max(0, width - len));
    }
    
    textLength(text) {
        // Account for emoji and special characters
        return text.replace(/[\u{1F300}-\u{1F9FF}]|[\u{2600}-\u{26FF}]/gu, '  ').length;
    }
}

// Initialize the TUI when page loads
document.addEventListener('DOMContentLoaded', () => {
    new LagrangeTUI();
});