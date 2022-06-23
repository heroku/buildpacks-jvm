require "rspec/core"
require "rspec/retry"

require "cutlass"

def test_dir
  Pathname(__dir__).join("../..")
end

JVM_FUNCTION_BUILDPACK = Cutlass::LocalBuildpack.new(directory: test_dir.join("meta-buildpacks/java-function"))
Cutlass.config do |config|
  config.default_buildpack_paths = [JVM_FUNCTION_BUILDPACK]
  config.default_builder = "heroku/buildpacks:18"
  config.default_repo_dirs = [test_dir.join("../test-fixtures")]
end

RSpec.configure do |config|
  # config.filter_run :focus => true

  config.before(:suite) do
    Cutlass::CleanTestEnv.record
  end

  config.after(:suite) do
    JVM_FUNCTION_BUILDPACK.teardown
    Cutlass::CleanTestEnv.check
  end
end
