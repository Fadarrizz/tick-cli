require_relative "lib/custom_download_strategy"

class TickCli < Formula
  version "0.1.2"
  desc "A CLI tool for managing Tickspot entries"
  homepage "https://github.com/Fadarrizz/tick-cli"
  url "https://github.com/Fadarrizz/tick-cli/releases/download/#{version}/tick-cli-#{version}-x86_64-apple-darwin.tar.gz", :using => GitHubPrivateRepositoryReleaseDownloadStrategy
  sha256 "0f69215686a900d51578be610d991e6d1b0456baad3eda2409fdb5aed0bc0241"
  head "https://github.com/Fadarrizz/tick-cli.git"

  def install
    bin.install "tick-cli"
  end
end
