require_relative "lib/oye_rust/version"

Gem::Specification.new do |spec|
  spec.name = "oye_rust"
  spec.version = OyeRust::VERSION
  spec.authors = ["Uchio Kondo"]
  spec.email = ["udzura@udzura.jp"]

  spec.summary = "Fukuokarb LT example."
  spec.description = "Fukuokarb LT example."
  spec.homepage = "https://udzura.jp"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.2.0.preview1"

  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (f == __FILE__) || \
      f.match(%r{\A(?:(?:bin|test|spec|features)/|\.(?:git|travis|circleci)|appveyor)})
    end + ["Cargo.lock"]
  end
  spec.require_paths = ["lib"]
  spec.extensions    = ["Cargo.toml"]
end
