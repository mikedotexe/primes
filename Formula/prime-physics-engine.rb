class PrimePhysicsEngine < Formula
  desc "High-performance membrane prime generator with mathematical foundations"
  homepage "https://github.com/mikepurvis/prime-physics-engine"
  url "https://github.com/mikepurvis/prime-physics-engine/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "fc0da7e8cf9151398cbbce4fff6c38ef796042a63c29266d729a73572bf00344"
  license "MIT"
  head "https://github.com/mikepurvis/prime-physics-engine.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args

    # Install examples as separate executables
    examples = %w[
      educational_explorer
      prime_discovery_dashboard
      membrane_lab_tui
      claude_md_claim_verifier
      concrete_prime_examples
      configuration_migration_tracker
      lagrange_point_verifier
    ]

    examples.each do |example|
      system "cargo", "build", "--release", "--example", example
      bin.install "target/release/examples/#{example}" => "prime-#{example.gsub('_', '-')}"
    end

    # Install documentation
    doc.install "README.md", "EVIDENCE.md", "VERIFIED_CLAIMS.md"
    doc.install "docs" if Dir.exist?("docs")
  end

  test do
    # Test basic functionality
    system "#{bin}/prime-physics-engine", "--version"
    
    # Test prime generation
    output = shell_output("#{bin}/prime-concrete-examples 2>&1")
    assert_match(/prime/i, output)
    
    # Run a simple verification
    system "#{bin}/prime-claude-md-claim-verifier", "--quick"
  end
end