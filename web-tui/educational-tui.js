// Enhanced Educational Lagrange TUI
// Shows larger primes with different padding densities and explains concepts

class EducationalLagrangeTUI {
    constructor() {
        this.state = {
            prime1: null,  // Moderate padding
            prime2: null,  // Heavy padding
            config1: { base: 10, outer: 3, inner: 7, k_outer: 2, k_inner: 1 },
            config2: { base: 10, outer: 3, inner: 7, k_outer: 4, k_inner: 2 },
            lagrangePoints: [],
            animationFrame: 0,
            isAnimating: false,
            primeSeparation: 100,
            currentExplanation: 0,
            explanations: [
                "Lagrange points are positions where gravitational forces balance perfectly.",
                "L1 between membrane primes preserves structure, averaging only the middle digit.",
                "For 303050303 and 303070303, L1 = 303060303 (middle: 6 = avg of 5,7).",
                "The membrane structure acts like a 'container' - only the center varies.",
                "This suggests the 'mass' is in the boundary digits, not the middle seed.",
                "In prime space, Lagrange points reveal the structural symmetries."
            ],
            showHelp: false,
            statusMessage: "Press 'g' to generate large membrane primes",
        };
        
        this.terminal = document.getElementById('terminal');
        this.setupEventListeners();
        this.render();
    }
    
    setupEventListeners() {
        document.addEventListener('keydown', (e) => {
            switch(e.key) {
                case 'g':
                    this.generateLargePrimes();
                    break;
                case 'a':
                    this.toggleAnimation();
                    break;
                case 'e':
                    this.cycleExplanation();
                    break;
                case 'h':
                case '?':
                    this.toggleHelp();
                    break;
                case 'q':
                    if (confirm('Quit the educational TUI?')) {
                        location.reload();
                    }
                    break;
            }
        });
        
        // Animation loop
        setInterval(() => {
            if (this.state.isAnimating) {
                this.animateStep();
                this.render();
            }
        }, 50);
    }
    
    generateLargePrimes() {
        // Generate primes with visible structure differences
        
        // Prime 1: Moderate padding - 3-00-7-0-17-0-7-00-3
        this.state.prime1 = {
            value: '3007001700703',
            bigValue: 3007001700703n,
            config: this.state.config1,
            visual: '3─◯◯─7─◯─1─7─◯─7─◯◯─3',
            structure: '3-◯◯-7-◯-[17]-◯-7-◯◯-3',
            breakdown: '3-2×◯-7-1×◯-[17]-1×◯-7-2×◯-3',
            mass: 25.4,
            position: [-50, 0]
        };
        
        // Prime 2: Heavy padding - 3-0000-7-00-23-00-7-0000-3
        this.state.prime2 = {
            value: '300007002300700003',
            bigValue: 300007002300700003n,
            config: this.state.config2,
            visual: '3─◯◯◯◯─7─◯◯─2─3─◯◯─7─◯◯◯◯─3',
            structure: '3-◯◯◯◯-7-◯◯-[23]-◯◯-7-◯◯◯◯-3',
            breakdown: '3-4×◯-7-2×◯-[23]-2×◯-7-4×◯-3',
            mass: 28.7,
            position: [50, 0]
        };
        
        this.calculateDetailedLagrangePoints();
        this.state.statusMessage = "Large primes generated! Press 'a' to animate, 'e' for explanations";
        
        // Flash effect
        this.terminal.classList.add('flash-success');
        setTimeout(() => this.terminal.classList.remove('flash-success'), 500);
    }
    
    calculateDetailedLagrangePoints() {
        if (!this.state.prime1 || !this.state.prime2) return;
        
        const p1 = this.state.prime1.bigValue;
        const p2 = this.state.prime2.bigValue;
        const massRatio = this.state.prime1.mass / (this.state.prime1.mass + this.state.prime2.mass);
        
        this.state.lagrangePoints = [
            {
                type: 'L1',
                name: 'Inner Lagrange',
                position: 0.5 - (0.2 * massRatio),
                value: this.interpolateValue(p1, p2, 0.5 - (0.2 * massRatio)),
                explanation: 'L1: Between primes - unstable equilibrium. Like a ball on a hill.',
                stability: 0.3,
                dominance: 'Balanced',
                symbol: '①'
            },
            {
                type: 'L2',
                name: 'Near-side',
                position: -0.2,
                value: this.extrapolateValue(p1, p2, -0.2),
                explanation: 'L2: Beyond smaller prime - shielded from larger prime.',
                stability: 0.2,
                dominance: 'Prime 1',
                symbol: '②'
            },
            {
                type: 'L3',
                name: 'Far-side',
                position: 1.2,
                value: this.extrapolateValue(p1, p2, 1.2),
                explanation: 'L3: Far side of larger prime - weakest influence.',
                stability: 0.1,
                dominance: 'Prime 2',
                symbol: '③'
            },
            {
                type: 'L4',
                name: 'Leading Trojan',
                position: 0.4,
                value: this.interpolateValue(p1, p2, 0.4),
                explanation: 'L4: Leading trojan - stable! Forms equilateral triangle.',
                stability: 0.8,
                dominance: 'Shared',
                symbol: '④'
            },
            {
                type: 'L5',
                name: 'Trailing Trojan',
                position: 0.6,
                value: this.interpolateValue(p1, p2, 0.6),
                explanation: 'L5: Trailing trojan - equally stable parking orbit.',
                stability: 0.8,
                dominance: 'Shared',
                symbol: '⑤'
            }
        ];
        
        // Check which are prime
        this.state.lagrangePoints.forEach(lp => {
            lp.isPrime = this.isPrime(lp.value);
        });
    }
    
    animateStep() {
        this.state.animationFrame = (this.state.animationFrame + 1) % 100;
        const t = this.state.animationFrame / 100;
        
        // Oscillate separation to show interaction
        this.state.primeSeparation = 80 + 20 * Math.sin(t * Math.PI * 2);
        
        // Update prime positions
        if (this.state.prime1) {
            this.state.prime1.position[0] = -this.state.primeSeparation / 2;
        }
        if (this.state.prime2) {
            this.state.prime2.position[0] = this.state.primeSeparation / 2;
        }
        
        // Recalculate Lagrange points
        this.calculateDetailedLagrangePoints();
    }
    
    toggleAnimation() {
        this.state.isAnimating = !this.state.isAnimating;
        this.state.statusMessage = this.state.isAnimating ? 
            "Animation started - watch the gravitational dance!" : 
            "Animation paused";
    }
    
    cycleExplanation() {
        this.state.currentExplanation = 
            (this.state.currentExplanation + 1) % this.state.explanations.length;
    }
    
    toggleHelp() {
        this.state.showHelp = !this.state.showHelp;
        if (this.state.showHelp) {
            alert(`Educational Lagrange TUI Help
            
This visualization shows how Lagrange points work in "prime space":

• Two large primes with different zero-padding densities
• Five Lagrange points where forces balance
• Animation shows gravitational interaction

Commands:
g - Generate large primes
a - Toggle animation
e - Cycle explanations
h - Show this help
q - Quit

The key insight: Just as planets have stable orbits, 
primes have mathematical "gravity" that creates 
equilibrium zones where other primes cluster.`);
        }
    }
    
    render() {
        const screen = this.buildEducationalScreen();
        this.terminal.textContent = screen;
    }
    
    buildEducationalScreen() {
        const width = 150;
        let screen = '';
        
        // Header
        screen += '┌' + '─'.repeat(width - 2) + '┐\n';
        screen += '│' + this.centerText('🎓 Lagrange Points in Prime Space - Understanding Gravitational Equilibrium', width - 2) + '│\n';
        screen += '└' + '─'.repeat(width - 2) + '┘\n';
        
        // Prime visualization section
        screen += '\n' + this.centerText('Membrane Primes with Different Zero-Padding Densities', width) + '\n\n';
        
        if (this.state.prime1 && this.state.prime2) {
            // Prime 1
            screen += '┌⚛️ Atom 1 (Moderate Padding)' + '─'.repeat(70) + '┐\n';
            screen += '│ Prime 1: ' + this.state.prime1.value + ' (13 digits)' + ' '.repeat(54) + '│\n';
            screen += '│ Config: (3,7) k=(2,1) base 10' + ' '.repeat(49) + '│\n';
            screen += '│ Visual: ' + this.state.prime1.visual + ' '.repeat(40) + '│\n';
            screen += '│ Structure: ' + this.state.prime1.breakdown + ' '.repeat(30) + '│\n';
            screen += '└' + '─'.repeat(79) + '┘\n\n';
            
            // Prime 2
            screen += '┌⚛️ Atom 2 (Heavy Padding)' + '─'.repeat(74) + '┐\n';
            screen += '│ Prime 2: ' + this.state.prime2.value + ' (18 digits)' + ' '.repeat(41) + '│\n';
            screen += '│ Config: (3,7) k=(4,2) base 10' + ' '.repeat(49) + '│\n';
            screen += '│ Visual: ' + this.state.prime2.visual + ' '.repeat(30) + '│\n';
            screen += '│ Structure: ' + this.state.prime2.breakdown + ' '.repeat(20) + '│\n';
            screen += '└' + '─'.repeat(79) + '┘\n\n';
            
            // Lagrange diagram
            screen += '┌🌌 Gravitational Map' + '─'.repeat(width - 22) + '┐\n';
            screen += '│' + ' '.repeat(width - 2) + '│\n';
            screen += '│   L2         P1                    L1                    P2         L3     │\n';
            screen += '│   ②          ●                     ①                     ●          ③      │\n';
            screen += '│   ├──────────┼─────────────────────┼─────────────────────┼──────────┤     │\n';
            screen += '│                           L4 ④ (above)                                     │\n';
            screen += '│                           L5 ⑤ (below)                                     │\n';
            screen += '│' + ' '.repeat(width - 2) + '│\n';
            
            // Show Lagrange point values
            this.state.lagrangePoints.forEach(lp => {
                const primeMarker = lp.isPrime ? ' ✓ PRIME!' : '';
                const line = `│ ${lp.symbol} ${lp.type}: ${lp.value}${primeMarker}`;
                screen += line + ' '.repeat(width - line.length - 1) + '│\n';
            });
            
            screen += '└' + '─'.repeat(width - 2) + '┘\n\n';
            
            // Explanation section
            screen += '┌💡 Understanding Lagrange Points' + '─'.repeat(width - 34) + '┐\n';
            const explanation = this.state.explanations[this.state.currentExplanation];
            screen += '│ ' + explanation + ' '.repeat(width - explanation.length - 3) + '│\n';
            screen += '│' + ' '.repeat(width - 2) + '│\n';
            screen += '│ ' + `(${this.state.currentExplanation + 1}/${this.state.explanations.length})` + 
                      ' Press "e" to cycle explanations' + ' '.repeat(width - 40) + '│\n';
            screen += '└' + '─'.repeat(width - 2) + '┘\n';
            
            // Animation indicator
            if (this.state.isAnimating) {
                const animBar = this.buildAnimationBar();
                screen += '\n' + this.centerText(animBar, width) + '\n';
            }
        } else {
            // No primes generated yet
            screen += '\n' + this.centerText('Press "g" to generate large membrane primes and see their interaction', width) + '\n';
        }
        
        // Status bar
        screen += '\n┌' + '─'.repeat(width - 2) + '┐\n';
        screen += '│Commands: g:generate a:animate e:explain h:help q:quit | ' + 
                  this.state.statusMessage + ' '.repeat(width - this.state.statusMessage.length - 60) + '│\n';
        screen += '└' + '─'.repeat(width - 2) + '┘\n';
        
        return screen;
    }
    
    buildAnimationBar() {
        const phase = this.state.animationFrame / 100;
        const barWidth = 40;
        const position = Math.floor(phase * barWidth);
        
        let bar = '[';
        for (let i = 0; i < barWidth; i++) {
            if (i === position) {
                bar += '●';
            } else if (Math.abs(i - position) < 3) {
                bar += '▪';
            } else {
                bar += '─';
            }
        }
        bar += '] Gravitational Oscillation';
        
        return bar;
    }
    
    interpolateValue(v1, v2, t) {
        // For membrane primes with same structure, preserve structure
        const s1 = v1.toString();
        const s2 = v2.toString();
        
        // Check if they have same length and structure
        if (Math.abs(t - 0.5) < 0.01 && s1.length === s2.length) {
            // Find differing positions
            const diffs = [];
            for (let i = 0; i < s1.length; i++) {
                if (s1[i] !== s2[i]) {
                    diffs.push(i);
                }
            }
            
            // If they differ in exactly one position (the middle)
            if (diffs.length === 1) {
                const pos = diffs[0];
                const d1 = parseInt(s1[pos]);
                const d2 = parseInt(s2[pos]);
                const avg = Math.floor((d1 + d2) / 2);
                
                // Create result preserving structure
                const result = s1.substring(0, pos) + avg + s1.substring(pos + 1);
                return BigInt(result);
            }
        }
        
        // Default: numeric interpolation
        const diff = v2 - v1;
        const offset = BigInt(Math.floor(Number(diff) * t));
        return v1 + offset;
    }
    
    extrapolateValue(v1, v2, t) {
        const diff = v2 - v1;
        if (t < 0) {
            const offset = BigInt(Math.floor(Number(diff) * Math.abs(t)));
            return v1 - offset;
        } else {
            const offset = BigInt(Math.floor(Number(diff) * (t - 1)));
            return v2 + offset;
        }
    }
    
    isPrime(n) {
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
    
    centerText(text, width) {
        const padding = Math.max(0, width - text.length);
        const leftPad = Math.floor(padding / 2);
        const rightPad = padding - leftPad;
        return ' '.repeat(leftPad) + text + ' '.repeat(rightPad);
    }
}

// Initialize when page loads
document.addEventListener('DOMContentLoaded', () => {
    new EducationalLagrangeTUI();
});