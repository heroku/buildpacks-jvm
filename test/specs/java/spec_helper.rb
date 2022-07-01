require "rspec/core"
require "rspec/retry"
require "java-properties"

require "cutlass"

def test_dir
  Pathname(__dir__).join("../..")
end

JVM_BUILDPACK = Cutlass::LocalBuildpack.new(directory: test_dir.join("meta-buildpacks/java"))
Cutlass.config do |config|
  config.default_buildpack_paths = [JVM_BUILDPACK]
  config.default_builder = "heroku/buildpacks:18"
  config.default_repo_dirs = [test_dir.join("../test-fixtures")]
end

RSpec.configure do |config|
  # config.filter_run :focus => true

  config.before(:suite) do
    Cutlass::CleanTestEnv.record
  end

  config.after(:suite) do
    JVM_BUILDPACK.teardown
    Cutlass::CleanTestEnv.check
  end
end

def remove_maven_wrapper(app_dir)
  File.delete("#{app_dir}/mvnw")
  File.delete("#{app_dir}/mvnw.cmd")
  FileUtils.remove_dir("#{app_dir}/.mvn/wrapper")
end

def set_java_version(app_dir, version_string)
  set_system_properties_key(app_dir, "java.runtime.version", version_string)
end

def set_maven_version(app_dir, version_string)
  set_system_properties_key(app_dir, "maven.version", version_string)
end

def set_system_properties_key(app_dir, key, value)
  properties = {}

  path = "#{app_dir}/system.properties"

  if File.file?(path)
    properties = JavaProperties.load(path)
  end

  properties[key.to_sym] = value
  JavaProperties.write(properties, path)
end
