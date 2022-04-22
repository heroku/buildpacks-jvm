require "rspec/core"
require "rspec/retry"
require "java-properties"

require "cutlass"

def root_dir
  Pathname(__dir__).join("../../..")
end

JVM_BUILDPACK = Cutlass::LocalBuildpack.new(directory: root_dir.join("buildpacks/jvm"))
MAVEN_BUILDPACK = Cutlass::LocalBuildpack.new(directory: root_dir.join("buildpacks/maven"))

Cutlass.config do |config|
  config.default_buildpack_paths = [JVM_BUILDPACK, MAVEN_BUILDPACK, "heroku/procfile:1.0.1"]
  config.default_builder = "heroku/buildpacks:20"
  config.default_repo_dirs = [root_dir.join("test/fixtures")]
end

RSpec.configure do |config|
  config.filter_run :focus => true
  config.run_all_when_everything_filtered = true

  config.before(:suite) do
    Cutlass::CleanTestEnv.record
  end

  config.after(:suite) do
    JVM_BUILDPACK.teardown
    MAVEN_BUILDPACK.teardown
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

def write_settings_xml(app_dir, filename, test_value)
  settings_xml = <<~EOF
        <settings xmlns="http://maven.apache.org/SETTINGS/1.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
          xsi:schemaLocation="http://maven.apache.org/SETTINGS/1.0.0 https://maven.apache.org/xsd/settings-1.0.0.xsd">

          <profiles>
              <profile>
                  <activation>
                      <activeByDefault>true</activeByDefault>
                  </activation>
                  <properties>
                      <heroku.maven.settings-test.value>#{test_value}</heroku.maven.settings-test.value>
                  </properties>
              </profile>
          </profiles>
        </settings>
  EOF

  File.open(File.join(app_dir, filename), "w") { |file| file.write(settings_xml) }
end
