require "rspec/core"
require "rspec/retry"
require "java-properties"
require_relative "../../rapier/rapier"

def rapier
  Rapier::Runner.new("test/fixtures", "heroku/buildpacks:18", default_buildpacks: ["./meta-buildpacks/java"])
end

RSpec.configure do |config|
  # config.filter_run :focus => true
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
