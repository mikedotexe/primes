// Lagrange TUI - WASM-powered version
// Uses the actual Rust engine compiled to WebAssembly

import init, { WasmLagrangeUI } from '../pkg/prime_physics_engine.js';

class LagrangeTUIWasm {
    constructor() {
        this.wasmUI = null;
        this.terminal = document.getElementById('terminal');
        this.helpOverlay = document.getElementById('help');
        this.initialized = false;
        
        // Initialize WASM
        this.initWasm();
    }
    
    async initWasm() {
        try {
            // Initialize the WASM module
            await init();
            
            // Create the WASM UI instance
            this.wasmUI = new WasmLagrangeUI();
            
            this.initialized = true;
            this.setupEventListeners();
            this.render();
            
            console.log('WASM TUI initialized successfully');
        } catch (error) {
            console.error('Failed to initialize WASM:', error);
            this.terminal.textContent = 'Failed to initialize WASM module: ' + error.message;
        }
    }
    
    setupEventListeners() {
        document.addEventListener('keydown', (e) => {
            if (!this.initialized) return;
            
            const state = this.wasmUI.getState();
            
            if (state.show_help) {
                this.wasmUI.toggleHelp();
                this.helpOverlay.style.display = 'none';
                this.render();
                return;
            }
            
            switch(e.key) {
                case 'g':
                    this.wasmUI.generatePrimePair();
                    this.flashSuccess();
                    this.render();
                    break;
                case 't':
                    this.wasmUI.testLagrangePoints();
                    this.render();
                    break;
                case 'c':
                    this.wasmUI.cycleConfiguration();
                    this.render();
                    break;
                case 'h':
                case '?':
                    this.wasmUI.toggleHelp();
                    this.helpOverlay.style.display = 'block';
                    break;
                case 'ArrowLeft':
                    this.wasmUI.selectPrime(0);
                    this.render();
                    break;
                case 'ArrowRight':
                    this.wasmUI.selectPrime(1);
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
    
    flashSuccess() {
        this.terminal.classList.add('flash-success');
        setTimeout(() => this.terminal.classList.remove('flash-success'), 500);
    }
    
    render() {
        if (!this.initialized) return;
        
        try {
            const state = this.wasmUI.getState();
            const screen = this.buildScreen(state);
            this.terminal.textContent = screen;
        } catch (error) {
            console.error('Render error:', error);
        }
    }
    
    buildScreen(state) {
        const width = 150;
        const height = 40;
        
        let screen = '';
        
        // Header
        screen += '┌' + '─'.repeat(width - 2) + '┐\n';
        screen += '│' + this.centerText('⚛️  Lagrange Point Explorer - Prime Atomic Interactions (WASM)', width - 2) + '│\n';
        screen += '└' + '─'.repeat(width - 2) + '┘\n';
        
        // Membrane Field
        screen += '┌⚛️ Membrane Field' + '─'.repeat(width - 19) + '┐\n';
        screen += '│' + this.centerText('╔═══════════════════════════════════════════════════════════════╗', width - 2) + '│\n';
        
        if (state.particle1 && state.particle2) {
            const p1Visual = state.particle1.visual || this.formatVisual(state.particle1.value);
            const p2Visual = state.particle2.visual || this.formatVisual(state.particle2.value);
            
            screen += '│' + this.centerText(`║ P₁: ${p1Visual} ║`, width - 2) + '│\n';
            screen += '│' + this.centerText(`║ P₂: ${p2Visual} ║`, width - 2) + '│\n';
            screen += '│' + this.centerText('╚═══════════════════════════════════════════════════════════════╝', width - 2) + '│\n';
            
            if (state.current_prime_distance) {
                const distBar = this.buildDistanceBar(state.current_prime_distance);
                screen += '│' + this.centerText(`║ P₁ ← ${state.current_prime_distance} → P₂ | Distance bar: ${distBar} ║`, width - 2) + '│\n';
            }
        } else {
            screen += '│' + this.centerText('║        Generate primes to see membrane interaction            ║', width - 2) + '│\n';
            screen += '│' + this.centerText('║                  Press \'g\' to begin                           ║', width - 2) + '│\n';
            screen += '│' + this.centerText('╚═══════════════════════════════════════════════════════════════╝', width - 2) + '│\n';
        }
        
        screen += '└' + '─'.repeat(width - 2) + '┘\n';
        
        // Main content area
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
                const atom1Content = this.getAtomContent(state.particle1, row - 1, 'Prime 1');
                const fieldContent = this.getFieldContent(state, row - 1);
                const atom2Content = this.getAtomContent(state.particle2, row - 1, 'Prime 2');
                
                screen += '│' + this.padRight(atom1Content, colWidth - 1) + '│';
                screen += '│' + this.padRight(fieldContent, colWidth - 1) + '│';
                screen += '│' + this.padRight(atom2Content, colWidth - 1) + '│\n';
            }
        }
        
        // Status bar
        const configStr = this.wasmUI.getConfigString();
        const statusMsg = this.wasmUI.getStatusMessage();
        screen += '┌' + '─'.repeat(width - 2) + '┐\n';
        screen += '│[' + configStr + '] ' + statusMsg;
        screen += ' | t:test c:config h:help q:quit';
        screen = screen.padEnd(screen.length + width - 2 - (screen.length - screen.lastIndexOf('\n') - 1)) + '│\n';
        screen += '└' + '─'.repeat(width - 2) + '┘\n';
        
        return screen;
    }
    
    formatVisual(value) {
        return value.split('').map((c, i, arr) => {
            if (c === '0' && i > 0 && i < arr.length - 1) {
                return '◯';
            }
            return c;
        }).join('─');
    }
    
    buildDistanceBar(distance) {
        const barWidth = 30;
        const maxDist = 10000;
        const dist = Math.min(parseInt(distance) || 0, maxDist);
        const fillAmount = Math.floor(dist * barWidth / maxDist);
        const filled = Math.min(Math.max(1, fillAmount), barWidth);
        const empty = barWidth - filled;
        return '[' + '█'.repeat(filled) + '░'.repeat(empty) + ']';
    }
    
    getAtomContent(particle, row, title) {
        if (!particle) return '';
        
        const lines = [
            this.centerText(title, 0),
            '',
            `Value: ${particle.value}`,
            `Structure: ${particle.structure}`,
            `Mass: ${particle.mass}`,
            `Base: ${particle.base}`
        ];
        
        return lines[row] || '';
    }
    
    getFieldContent(state, row) {
        const lines = ['╔═══ Lagrange Analysis ═══╗'];
        
        if (state.particle1 && state.particle2) {
            lines.push('');
            lines.push(`Range: ${state.particle1.value} ↔ ${state.particle2.value}`);
            
            if (state.lagrange_points && state.lagrange_points.length > 0) {
                const l1 = state.lagrange_points[0];
                const primeIndicator = l1.is_prime ? ' ✓ PRIME!' : '';
                lines.push(`L₁ (midpoint): ${l1.value}${primeIndicator}`);
                lines.push('');
                lines.push(`L1: (${l1.position[0].toFixed(1)}, ${l1.position[1].toFixed(1)})`);
                lines.push(`Field: ${l1.field_strength.toFixed(2)} | Stability: ${l1.stability.toFixed(2)}`);
                
                if (l1.tested) {
                    lines.push(l1.is_prime ? '✓ Tested - PRIME!' : '✓ Tested - Composite');
                }
            }
        }
        
        return this.centerText(lines[row] || '', 0);
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
        return text.replace(/[\u{1F300}-\u{1F9FF}]|[\u{2600}-\u{26FF}]/gu, '  ').length;
    }
}

// Initialize when page loads
document.addEventListener('DOMContentLoaded', () => {
    new LagrangeTUIWasm();
});