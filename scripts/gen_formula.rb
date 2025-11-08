#!/usr/bin/env ruby
# Generate Homebrew formula for Prime Physics Engine

require 'optparse'

options = {}
OptionParser.new do |opts|
  opts.banner = "Usage: gen_formula.rb [options]"
  
  opts.on("-v", "--version VERSION", "Version number") do |v|
    options[:version] = v
  end
  
  opts.on("-m", "--sha-mac SHA", "SHA256 for macOS") do |s|
    options[:sha_mac] = s
  end
  
  opts.on("-l", "--sha-linux SHA", "SHA256 for Linux") do |s|
    options[:sha_linux] = s
  end
end.parse!

version = options[:version] || "1.0.0"
sha_mac = options[:sha_mac] || "0" * 64
sha_linux = options[:sha_linux] || "0" * 64

formula = <<~RUBY
class PrimePhysicsEngine < Formula
  desc "High-performance prime number generation using membrane physics"
  homepage "https://github.com/mikepurvis/prime-physics-engine"
  version "#{version}"
  license "MIT"

  on_macos do
    url "https://github.com/mikepurvis/prime-physics-engine/releases/download/v#{version}/prime-physics-engine-#{version}-x86_64-apple-darwin.tar.gz"
    sha256 "#{sha_mac}"
  end

  on_linux do
    url "https://github.com/mikepurvis/prime-physics-engine/releases/download/v#{version}/prime-physics-engine-#{version}-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "#{sha_linux}"
  end

  depends_on "rust" => :build

  def install
    bin.install "prime-physics-engine"
    
    # Install examples
    (share/"examples").install Dir["examples/verified/*"]
    
    # Install documentation
    doc.install "README.md", "CHANGELOG.md"
    (doc/"guides").install Dir["docs/*.md"]
  end

  test do
    # Test basic functionality
    assert_match "Prime Physics Engine", shell_output("\#{bin}/prime-physics-engine --version")
    
    # Test prime generation
    output = shell_output("\#{bin}/prime-physics-engine --base 6 --config 1,5 --count 10")
    assert_match(/Found \d+ primes/, output)
  end
end
RUBY

puts formula