# LaTeX Figure Code

## TikZ Figures for the Paper

### Figure 1: Membrane Structure Visualization

```latex
\begin{figure}[ht]
\centering
\begin{tikzpicture}[scale=1.2]
    % Traditional approach
    \node[align=center] at (0, 3) {\textbf{Traditional Prime Search}};
    \foreach \x in {0,...,9} {
        \node at (\x*0.8-3.2, 2) {?};
        \draw[->] (\x*0.8-3.2, 1.8) -- (\x*0.8-3.2, 1.2);
        \node at (\x*0.8-3.2, 0.8) {\footnotesize \pgfmathparse{int(101+\x)}\pgfmathresult};
        \node at (\x*0.8-3.2, 0.2) {
            \pgfmathparse{int(mod(101+\x,2)*mod(101+\x,3)*mod(101+\x,5))}
            \ifnum\pgfmathresult=0 ✗\else
                \pgfmathparse{isprime(101+\x)}\ifnum\pgfmathresult=1 ✓\else ✗\fi
            \fi
        };
    }
    
    % Membrane approach
    \node[align=center] at (0, -1.5) {\textbf{Membrane Prime Search (Base 6)}};
    \node at (-3.2, -2.5) {$L$}; \node at (-2.4, -2.5) {0};
    \node at (-1.6, -2.5) {$R$}; \node at (-0.8, -2.5) {0};
    \node[fill=yellow!30] at (0, -2.5) {$C$};
    \node at (0.8, -2.5) {0}; \node at (1.6, -2.5) {$R$};
    \node at (2.4, -2.5) {0}; \node at (3.2, -2.5) {$L$};
    
    \draw[->] (0, -2.8) -- (0, -3.4);
    \node at (0, -3.7) {$M(C) = 5 \cdot 6^4 + 5 \cdot 6^2 + C \cdot 6 + 5 = 6485 + 6C$};
    \node at (0, -4.3) {For $C=2$: $M(2) = 6497$ \checkmark prime!};
    
    % Success rates
    \node[draw,fill=red!20] at (-4, 0.5) {$\sim$10\% success};
    \node[draw,fill=green!20] at (4, -3) {25-30\% success};
\end{tikzpicture}
\caption{Traditional sequential search tests arbitrary numbers with $\sim$10\% success rate. Membrane polynomials create structured numbers achieving 25-30\% prime density.}
\label{fig:membrane-structure}
\end{figure}
```

### Figure 2: Affine Transform Visualization

```latex
\begin{figure}[ht]
\centering
\begin{tikzpicture}[scale=0.9]
    % Before box
    \node[draw,thick,minimum width=5cm,minimum height=3cm,align=left] (before) at (0,0) {
        \textbf{Before (expensive):}\\
        $M(c) = 245 + 6c$\\
        $M(0) = 245 \div 7 = 35$ R $0$ ✗\\
        $M(1) = 251 \div 7 = 35$ R $6$ ✓\\
        $M(2) = 257 \div 7 = 36$ R $5$ ✓\\
        Complex division each time
    };
    
    % Arrow
    \draw[->,ultra thick] (3,0) -- (5,0) node[midway,above] {Affine Transform};
    
    % After box
    \node[draw,thick,minimum width=5cm,minimum height=3cm,align=left] (after) at (8,0) {
        \textbf{After (linear):}\\
        $s=0, g=6$ (precomputed)\\
        $c=0: (0+0 \times 6) \bmod 7 = 0$ ✗\\
        $c=1: (0+1 \times 6) \bmod 7 = 6$ ✓\\
        $c=2: (0+2 \times 6) \bmod 7 = 5$ ✓\\
        Simple multiply-add pattern
    };
    
    % Performance indicators
    \node[red] at (0,-2.2) {$\sim$20 cycles/test};
    \node[green] at (8,-2.2) {$\sim$3 cycles/test};
\end{tikzpicture}
\caption{The affine transform converts expensive modular division into predictable multiply-add operations, enabling GPU parallelization.}
\label{fig:affine-transform}
\end{figure}
```

### Figure 3: Residue Space Trajectories

```latex
\begin{figure}[ht]
\centering
\begin{tikzpicture}[scale=1.5]
    % Axes
    \draw[->] (-0.5,0) -- (5.5,0) node[right] {mod 3};
    \draw[->] (0,-0.5) -- (0,5.5) node[above] {mod 5};
    
    % Grid
    \foreach \x in {0,...,4} {
        \foreach \y in {0,...,4} {
            \node[circle,fill=gray!20,inner sep=1pt] at (\x,\y) {};
        }
    }
    
    % Divisibility walls
    \draw[red,very thick] (-0.3,0) -- (5.3,0) node[right] {divisible by 5};
    \draw[red,very thick] (0,-0.3) -- (0,5.3) node[above] {divisible by 3};
    
    % Membrane trajectory
    \draw[blue,ultra thick,->,
          decoration={markings,
          mark=at position 0.2 with {\node[above] {$M(0)$};},
          mark=at position 0.4 with {\node[above] {$M(1)$};},
          mark=at position 0.6 with {\node[above] {$M(2)$};},
          mark=at position 0.8 with {\node[above] {$M(3)$};},
          },postaction={decorate}] 
          plot[smooth] coordinates {(2,0) (2,1) (2,2) (2,3) (2,4)};
    
    % Random trajectory for comparison
    \draw[gray,dashed] plot[smooth] coordinates {(1,1) (0,3) (3,2) (1,4) (4,0)};
    
    % Legend
    \node[blue] at (3.5,4.5) {Membrane path};
    \node[gray] at (3.5,4) {Random path};
\end{tikzpicture}
\caption{Membrane sequences trace linear paths through residue space (blue), systematically avoiding divisibility walls (red) where coordinates equal zero. Random sequences (gray) hit walls frequently.}
\label{fig:residue-space}
\end{figure}
```

### Figure 4: GPU Performance Evolution

```latex
\begin{figure}[ht]
\centering
\begin{tikzpicture}[scale=0.8]
    \begin{axis}[
        xlabel={Optimization Stage},
        ylabel={Throughput (M candidates/s)},
        ymode=log,
        ymin=0.1,
        ymax=300,
        xtick={0,1,2,3,4,5,6,7},
        xticklabels={CPU,GPU,Affine,Thread,Batch,SIMD,Recip,Final},
        legend pos=north west,
        grid=major,
        width=12cm,
        height=8cm
    ]
    
    \addplot[color=blue,mark=*,thick] coordinates {
        (0,0.27) (1,0.297) (2,3.0) (3,10.5) (4,30.8) (5,51.9) (6,93.0) (7,186.9)
    };
    
    % Annotations for key improvements
    \node[pin=45:{10x: Remove branches}] at (axis cs:2,3.0) {};
    \node[pin=45:{3.5x: Shared memory}] at (axis cs:3,10.5) {};
    \node[pin=45:{1.8x: Fast modulo}] at (axis cs:6,93.0) {};
    
    \end{axis}
\end{tikzpicture}
\caption{Throughput evolution from 270k to 186.9M candidates/second. Each optimization enables the next, creating superlinear speedup (691x total).}
\label{fig:performance-evolution}
\end{figure}
```

### Figure 5: Breathing Pattern Comparison

```latex
\begin{figure}[ht]
\centering
\begin{tikzpicture}[scale=1]
    % Symmetric pattern
    \begin{scope}[shift={(0,0)}]
        \node at (0,1.5) {\textbf{Symmetric $k=(1,1)$}};
        \foreach \x/\c in {0/3,1/0,2/7,3/0,4/C,5/0,6/7,7/0,8/3} {
            \ifnum\x=4
                \node[circle,fill=yellow!50,draw] at (\x*0.6,0) {\c};
            \else
                \node[circle,fill=blue!20,draw] at (\x*0.6,0) {\c};
            \fi
        }
        \node at (2.4,-1) {Density: 21.3\%};
    \end{scope}
    
    % Breathing pattern
    \begin{scope}[shift={(7,0)}]
        \node at (0,1.5) {\textbf{Breathing $k=(0,1)$}};
        \foreach \x/\c in {0/3,1/7,2/0,3/C,4/0,5/7,6/3} {
            \ifnum\x=3
                \node[circle,fill=yellow!50,draw] at (\x*0.8,0) {\c};
            \else
                \node[circle,fill=green!20,draw] at (\x*0.8,0) {\c};
            \fi
        }
        \node at (2.4,-1) {Density: 30.2\%};
        \node[green] at (2.4,-1.5) {+42\% improvement!};
    \end{scope}
\end{tikzpicture}
\caption{Asymmetric "breathing" patterns achieve significantly higher prime density than symmetric configurations by creating favorable phase relationships in residue space.}
\label{fig:breathing-pattern}
\end{figure}
```

### Figure 6: Complete Pipeline Timing

```latex
\begin{figure}[ht]
\centering
\begin{tikzpicture}[scale=1]
    % Timeline
    \draw[thick] (0,0) -- (12,0);
    
    % CPU Phase 1
    \draw[fill=blue!30] (0,0.2) rectangle (0.5,1.2);
    \node[above] at (0.25,1.2) {CPU};
    \node[below] at (0.25,0) {2.7ms};
    \node at (0.25,0.7) {\rotatebox{90}{\tiny Membrane}};
    
    % Transfer 1
    \draw[fill=gray!30] (0.5,0.2) rectangle (0.6,0.8);
    \node[below] at (0.55,0) {0.3ms};
    
    % GPU Phase
    \draw[fill=green!30] (0.6,0.2) rectangle (3,1.5);
    \node[above] at (1.8,1.5) {GPU};
    \node[below] at (1.8,0) {21.4ms};
    \node at (1.8,0.85) {Sieve};
    
    % Transfer 2
    \draw[fill=gray!30] (3,0.2) rectangle (3.1,0.8);
    \node[below] at (3.05,0) {0.2ms};
    
    % CPU Phase 2
    \draw[fill=blue!30] (3.1,0.2) rectangle (6,1.2);
    \node[above] at (4.55,1.2) {CPU};
    \node[below] at (4.55,0) {42.6ms};
    \node at (4.55,0.7) {Miller-Rabin};
    
    % Total time
    \draw[<->] (0,-0.8) -- (6,-0.8) node[midway,below] {Total: 66.7ms = 60M candidates/s};
    
    % Data sizes
    \node[above] at (0.55,0.8) {\tiny 16MB};
    \node[above] at (3.05,0.8) {\tiny 100KB};
\end{tikzpicture}
\caption{End-to-end pipeline leverages each processor's strengths. Small transfer sizes (100KB return) minimize PCIe overhead.}
\label{fig:pipeline-timing}
\end{figure}
```

### Compiling Figures

```latex
\documentclass{article}
\usepackage{tikz}
\usepackage{pgfplots}
\pgfplotsset{compat=1.17}
\usetikzlibrary{decorations.markings,patterns,arrows.meta}

% In document:
\input{figures/membrane-structure}
\input{figures/affine-transform}
% etc.
```

These TikZ figures provide:
- Clear visual explanation of concepts
- Professional appearance for publication
- Exact control over layout and styling
- Vector graphics that scale perfectly
- Consistent visual language throughout the paper

Each figure is designed to be self-contained and immediately understandable, supporting the narrative flow of the paper.