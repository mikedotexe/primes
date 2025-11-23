#!/usr/bin/env python3
"""
Prime Fingerprint Visualization

Creates 2D projections of the 111-dimensional fingerprint space using:
- t-SNE (nonlinear, preserves local structure)
- PCA (linear, preserves global variance)
- UMAP (optional, best of both worlds)

Usage:
    python plot_fingerprints.py fingerprints/fingerprints.csv
"""

import sys
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns
from sklearn.decomposition import PCA
from sklearn.manifold import TSNE

# Optional: UMAP (install with: pip install umap-learn)
try:
    import umap
    HAS_UMAP = True
except ImportError:
    HAS_UMAP = False

def load_data(csv_path):
    """Load fingerprint CSV and separate features/labels"""
    df = pd.read_csv(csv_path)
    labels = df['label']
    features = df.drop(['label', 'sample_size'], axis=1)
    return features, labels, df

def plot_tsne(X, y, perplexity=30, max_iter=1000):
    """Generate t-SNE projection"""
    print(f"\nComputing t-SNE (perplexity={perplexity}, iterations={max_iter})...")
    tsne = TSNE(n_components=2, perplexity=perplexity, max_iter=max_iter, random_state=42, verbose=1)
    X_tsne = tsne.fit_transform(X)

    plt.figure(figsize=(14, 10))
    unique_labels = np.unique(y)
    colors = sns.color_palette('husl', len(unique_labels))

    for i, label in enumerate(unique_labels):
        mask = y == label
        plt.scatter(X_tsne[mask, 0], X_tsne[mask, 1],
                   c=[colors[i]], label=label, s=100, alpha=0.7, edgecolors='black')

    plt.xlabel('t-SNE Dimension 1', fontsize=12)
    plt.ylabel('t-SNE Dimension 2', fontsize=12)
    plt.title('Prime Constructor Fingerprints (t-SNE Projection)', fontsize=14, fontweight='bold')
    plt.legend(bbox_to_anchor=(1.05, 1), loc='upper left', fontsize=9)
    plt.grid(alpha=0.3)
    plt.tight_layout()
    plt.savefig('fingerprint_tsne.png', dpi=300, bbox_inches='tight')
    print("📊 t-SNE plot saved to: fingerprint_tsne.png")

def plot_pca(X, y):
    """Generate PCA projection"""
    print("\nComputing PCA...")
    pca = PCA(n_components=2)
    X_pca = pca.fit_transform(X)

    print(f"  Explained variance: PC1={pca.explained_variance_ratio_[0]:.3f}, PC2={pca.explained_variance_ratio_[1]:.3f}")
    print(f"  Total variance captured: {pca.explained_variance_ratio_.sum():.3f}")

    plt.figure(figsize=(14, 10))
    unique_labels = np.unique(y)
    colors = sns.color_palette('husl', len(unique_labels))

    for i, label in enumerate(unique_labels):
        mask = y == label
        plt.scatter(X_pca[mask, 0], X_pca[mask, 1],
                   c=[colors[i]], label=label, s=100, alpha=0.7, edgecolors='black')

    plt.xlabel(f'PC1 ({pca.explained_variance_ratio_[0]:.1%} variance)', fontsize=12)
    plt.ylabel(f'PC2 ({pca.explained_variance_ratio_[1]:.1%} variance)', fontsize=12)
    plt.title('Prime Constructor Fingerprints (PCA Projection)', fontsize=14, fontweight='bold')
    plt.legend(bbox_to_anchor=(1.05, 1), loc='upper left', fontsize=9)
    plt.grid(alpha=0.3)
    plt.tight_layout()
    plt.savefig('fingerprint_pca.png', dpi=300, bbox_inches='tight')
    print("📊 PCA plot saved to: fingerprint_pca.png")

def plot_umap(X, y, n_neighbors=15, min_dist=0.1):
    """Generate UMAP projection (if available)"""
    if not HAS_UMAP:
        print("\n⚠️  UMAP not available. Install with: pip install umap-learn")
        return

    print(f"\nComputing UMAP (neighbors={n_neighbors}, min_dist={min_dist})...")
    reducer = umap.UMAP(n_neighbors=n_neighbors, min_dist=min_dist, random_state=42, verbose=True)
    X_umap = reducer.fit_transform(X)

    plt.figure(figsize=(14, 10))
    unique_labels = np.unique(y)
    colors = sns.color_palette('husl', len(unique_labels))

    for i, label in enumerate(unique_labels):
        mask = y == label
        plt.scatter(X_umap[mask, 0], X_umap[mask, 1],
                   c=[colors[i]], label=label, s=100, alpha=0.7, edgecolors='black')

    plt.xlabel('UMAP Dimension 1', fontsize=12)
    plt.ylabel('UMAP Dimension 2', fontsize=12)
    plt.title('Prime Constructor Fingerprints (UMAP Projection)', fontsize=14, fontweight='bold')
    plt.legend(bbox_to_anchor=(1.05, 1), loc='upper left', fontsize=9)
    plt.grid(alpha=0.3)
    plt.tight_layout()
    plt.savefig('fingerprint_umap.png', dpi=300, bbox_inches='tight')
    print("📊 UMAP plot saved to: fingerprint_umap.png")

def plot_pairwise_distances(X, y):
    """Plot pairwise distance matrix between constructor centroids"""
    print("\nComputing pairwise distances...")

    unique_labels = np.unique(y)
    centroids = []
    for label in unique_labels:
        mask = y == label
        centroid = X[mask].mean(axis=0)
        centroids.append(centroid)

    centroids = np.array(centroids)

    # Compute pairwise Euclidean distances
    n = len(unique_labels)
    dist_matrix = np.zeros((n, n))
    for i in range(n):
        for j in range(n):
            dist_matrix[i, j] = np.linalg.norm(centroids[i] - centroids[j])

    plt.figure(figsize=(12, 10))
    sns.heatmap(dist_matrix, annot=True, fmt='.1f', cmap='viridis',
                xticklabels=unique_labels, yticklabels=unique_labels,
                cbar_kws={'label': 'Euclidean Distance'})
    plt.title('Pairwise Distances Between Constructor Centroids', fontsize=14, fontweight='bold')
    plt.xticks(rotation=45, ha='right')
    plt.yticks(rotation=0)
    plt.tight_layout()
    plt.savefig('pairwise_distances.png', dpi=300)
    print("📊 Pairwise distance matrix saved to: pairwise_distances.png")

def analyze_variance_by_component(X, y, n_components=10):
    """Show variance captured by each PCA component"""
    print(f"\nAnalyzing variance distribution (top {n_components} components)...")

    pca = PCA(n_components=n_components)
    pca.fit(X)

    plt.figure(figsize=(10, 6))
    plt.bar(range(1, n_components + 1), pca.explained_variance_ratio_)
    plt.xlabel('Principal Component')
    plt.ylabel('Variance Explained')
    plt.title('Variance Captured by PCA Components')
    plt.xticks(range(1, n_components + 1))
    plt.grid(axis='y', alpha=0.3)

    # Add cumulative variance line
    cumsum = np.cumsum(pca.explained_variance_ratio_)
    ax2 = plt.twinx()
    ax2.plot(range(1, n_components + 1), cumsum, 'r-o', linewidth=2, markersize=6)
    ax2.set_ylabel('Cumulative Variance', color='r')
    ax2.tick_params(axis='y', labelcolor='r')

    plt.tight_layout()
    plt.savefig('variance_analysis.png', dpi=300)
    print("📊 Variance analysis saved to: variance_analysis.png")

    print(f"\nCumulative variance:")
    for i, var in enumerate(cumsum):
        print(f"  PC1-{i+1}: {var:.3f}")

def main(csv_path):
    print("🎨 Prime Fingerprint Visualization")
    print("=" * 80)

    # Load data
    print(f"\nLoading data from: {csv_path}")
    X, y, df = load_data(csv_path)

    print(f"\nDataset Summary:")
    print(f"  Total samples: {len(y)}")
    print(f"  Features: {X.shape[1]}")
    print(f"  Constructors: {len(np.unique(y))}")

    # Generate visualizations
    plot_pca(X, y)

    # Adjust perplexity for small sample size (must be < n_samples)
    perplexity = min(5, len(y) - 1)
    plot_tsne(X, y, perplexity=perplexity)
    plot_umap(X, y)
    plot_pairwise_distances(X.values, y.values)
    analyze_variance_by_component(X, y)

    print("\n" + "=" * 80)
    print("SUMMARY")
    print("=" * 80)
    print("✓ Generated 5 plots:")
    print("  - fingerprint_pca.png (linear projection)")
    print("  - fingerprint_tsne.png (nonlinear, local structure)")
    if HAS_UMAP:
        print("  - fingerprint_umap.png (nonlinear, global + local)")
    print("  - pairwise_distances.png (centroid distances)")
    print("  - variance_analysis.png (PCA variance breakdown)")

    print("\n💡 Interpretation tips:")
    print("  - Tight clusters → similar fingerprints")
    print("  - Distant points → distinct construction methods")
    print("  - Overlap → classifier may confuse these")
    print("  - PCA variance → how much structure is linear vs nonlinear")

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <fingerprints.csv>")
        sys.exit(1)

    csv_path = sys.argv[1]
    main(csv_path)
