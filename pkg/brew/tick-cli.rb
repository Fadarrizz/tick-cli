class TickCli < Formula
  version "0.1.2"
  desc "A CLI tool for managing Tickspot entries"
  homepage "https://github.com/Fadarrizz/tick-cli"
  url "https://github.com/BurntSushi/ripgrep/releases/download/#{version}/ripgrep-#{version}-x86_64-apple-darwin.tar.gz"
  sha256 "585c18350cb8d4392461edd6c921e6edd5a97cbfc03b567d7bd440423e118082"

  def install
    bin.install "tc"
  end
end
