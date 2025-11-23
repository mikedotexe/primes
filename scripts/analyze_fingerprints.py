#!/usr/bin/env python3
"""
Prime Constructor Classifier

Trains a Random Forest classifier to distinguish prime construction methods
based on their modular/spectral fingerprints.

Usage:
    python analyze_fingerprints.py fingerprints/fingerprints.csv
"""

import sys
import pandas as pd
import numpy as np
from sklearn.ensemble import RandomForestClassifier
from sklearn.model_selection import train_test_split, cross_val_score
from sklearn.metrics import classification_report, confusion_matrix
import matplotlib.pyplot as plt
import seaborn as sns

def load_data(csv_path):
    """Load fingerprint CSV and separate features/labels"""
    df = pd.read_csv(csv_path)

    # Extract label and sample size
    labels = df['label']
    sample_sizes = df['sample_size']

    # All other columns are features
    features = df.drop(['label', 'sample_size'], axis=1)

    return features, labels, sample_sizes, df

def train_classifier(X, y, test_size=0.3, random_state=42):
    """Train Random Forest classifier with cross-validation"""
    X_train, X_test, y_train, y_test = train_test_split(
        X, y, test_size=test_size, random_state=random_state, stratify=y
    )

    # Train classifier
    clf = RandomForestClassifier(
        n_estimators=100,
        max_depth=10,
        random_state=random_state,
        n_jobs=-1
    )

    clf.fit(X_train, y_train)

    # Predictions
    y_pred = clf.predict(X_test)

    # Cross-validation score
    cv_scores = cross_val_score(clf, X, y, cv=5, n_jobs=-1)

    return clf, X_train, X_test, y_train, y_test, y_pred, cv_scores

def analyze_feature_importance(clf, feature_names, top_n=20):
    """Analyze and plot feature importance"""
    importances = clf.feature_importances_
    indices = np.argsort(importances)[::-1]

    print("\n" + "="*80)
    print(f"TOP {top_n} MOST IMPORTANT FEATURES")
    print("="*80)

    for i, idx in enumerate(indices[:top_n]):
        print(f"{i+1:3d}. {feature_names[idx]:30s} {importances[idx]:.6f}")

    # Plot
    plt.figure(figsize=(12, 8))
    plt.barh(range(top_n), importances[indices[:top_n][::-1]])
    plt.yticks(range(top_n), [feature_names[i] for i in indices[:top_n][::-1]])
    plt.xlabel('Feature Importance')
    plt.title('Top Feature Importances')
    plt.tight_layout()
    plt.savefig('feature_importance.png', dpi=300)
    print(f"\n📊 Feature importance plot saved to: feature_importance.png")

    return importances, indices

def plot_confusion_matrix(y_test, y_pred, labels):
    """Plot confusion matrix"""
    cm = confusion_matrix(y_test, y_pred, labels=np.unique(labels))

    plt.figure(figsize=(12, 10))
    sns.heatmap(cm, annot=True, fmt='d', cmap='Blues',
                xticklabels=np.unique(labels),
                yticklabels=np.unique(labels))
    plt.xlabel('Predicted')
    plt.ylabel('True')
    plt.title('Confusion Matrix')
    plt.xticks(rotation=45, ha='right')
    plt.yticks(rotation=0)
    plt.tight_layout()
    plt.savefig('confusion_matrix.png', dpi=300)
    print(f"📊 Confusion matrix saved to: confusion_matrix.png")

def analyze_modular_features(clf, feature_names, importances):
    """Analyze which moduli are most informative"""
    modular_features = {}

    for modulus in [3, 7, 11, 13, 17, 19]:
        pattern = f'mod{modulus}_'
        mod_features = [i for i, name in enumerate(feature_names) if pattern in name]
        total_importance = sum(importances[i] for i in mod_features)
        modular_features[modulus] = total_importance

    print("\n" + "="*80)
    print("MODULAR FEATURE IMPORTANCE BY MODULUS")
    print("="*80)

    for modulus in sorted(modular_features.keys()):
        print(f"Mod {modulus:2d}: {modular_features[modulus]:.6f}")

    # Plot
    plt.figure(figsize=(10, 6))
    moduli = list(modular_features.keys())
    importances_by_mod = [modular_features[m] for m in moduli]

    plt.bar(moduli, importances_by_mod)
    plt.xlabel('Modulus')
    plt.ylabel('Total Feature Importance')
    plt.title('Feature Importance by Modulus')
    plt.xticks(moduli)
    plt.grid(axis='y', alpha=0.3)
    plt.savefig('modular_importance.png', dpi=300)
    print(f"\n📊 Modular importance plot saved to: modular_importance.png")

def main(csv_path):
    print("🎯 Prime Constructor Classifier")
    print("=" * 80)

    # Load data
    print(f"\nLoading data from: {csv_path}")
    X, y, sample_sizes, df = load_data(csv_path)

    print(f"\nDataset Summary:")
    print(f"  Total samples: {len(y)}")
    print(f"  Number of features: {X.shape[1]}")
    print(f"  Constructor classes: {len(np.unique(y))}")
    print(f"\nClass distribution:")
    for label in np.unique(y):
        count = sum(y == label)
        print(f"  {label}: {count} samples")

    # Train classifier
    print("\n" + "="*80)
    print("TRAINING CLASSIFIER")
    print("="*80)

    clf, X_train, X_test, y_train, y_test, y_pred, cv_scores = train_classifier(X, y)

    print(f"\nCross-validation scores: {cv_scores}")
    print(f"Mean CV accuracy: {cv_scores.mean():.4f} (+/- {cv_scores.std() * 2:.4f})")

    # Test set evaluation
    test_accuracy = (y_pred == y_test).mean()
    print(f"Test set accuracy: {test_accuracy:.4f}")

    print("\n" + "="*80)
    print("CLASSIFICATION REPORT")
    print("="*80)
    print(classification_report(y_test, y_pred))

    # Feature importance
    importances, indices = analyze_feature_importance(clf, X.columns.tolist())

    # Modular analysis
    analyze_modular_features(clf, X.columns.tolist(), importances)

    # Confusion matrix
    print("\n" + "="*80)
    print("CONFUSION MATRIX")
    print("="*80)
    plot_confusion_matrix(y_test, y_pred, y)

    # Summary
    print("\n" + "="*80)
    print("SUMMARY")
    print("="*80)
    print(f"✓ Classifier can distinguish construction methods with {test_accuracy*100:.1f}% accuracy")
    print(f"✓ Cross-validation mean: {cv_scores.mean()*100:.1f}%")
    print(f"✓ Generated 3 plots: feature_importance.png, modular_importance.png, confusion_matrix.png")

    if test_accuracy > 0.8:
        print("\n🎉 High accuracy! Construction methods have distinct modular fingerprints.")
    elif test_accuracy > 0.5:
        print("\n👍 Moderate accuracy. Some construction methods are distinguishable.")
    else:
        print("\n⚠️  Low accuracy. Fingerprints may not be distinctive enough.")

    print("\n💡 Next steps:")
    print("  - Inspect feature_importance.png to see which features matter most")
    print("  - Check confusion_matrix.png to see which methods are confused")
    print("  - Try plot_fingerprints.py for visualization in feature space")

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <fingerprints.csv>")
        sys.exit(1)

    csv_path = sys.argv[1]
    main(csv_path)
