require_relative "spec_helper"

DEFAULT_MAVEN_VERSION = "3.6.2"
PREVIOUS_MAVEN_VERSION = "3.5.4"
UNKNOWN_MAVEN_VERSION = "1.0.0-unknown-version"
SIMPLE_HTTP_SERVICE_MAVEN_WRAPPER_VERSION = "3.6.3"

describe "Heroku's Maven Cloud Native Buildpack" do
  context "for an app with Maven wrapper" do
    it "will use Maven wrapper to build the app" do
      Cutlass::App.new("simple-http-service").transaction do |app|
        app.pack_build do |pack_result|
          expect(pack_result.stdout).to_not include("Selected Maven version:")
          expect(pack_result.stdout).to include("Maven wrapper detected, skipping installation.")
          expect(pack_result.stdout).to include("$ ./mvnw")
          expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - MAVEN VERSION] #{SIMPLE_HTTP_SERVICE_MAVEN_WRAPPER_VERSION}")
        end
      end
    end

    context "that also has 'maven.version=#{PREVIOUS_MAVEN_VERSION}' in its system.properties file" do
      it "will install and use Maven #{PREVIOUS_MAVEN_VERSION}" do
        Cutlass::App.new("simple-http-service").transaction do |app|
          set_maven_version(app.tmpdir, PREVIOUS_MAVEN_VERSION)
          app.pack_build do |pack_result|
            expect(pack_result.stdout).to include("Selected Maven version: #{PREVIOUS_MAVEN_VERSION}")
            expect(pack_result.stdout).to_not include("$ ./mvnw")
            expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - MAVEN VERSION] #{PREVIOUS_MAVEN_VERSION}")
          end
        end
      end
    end

    context "that also has 'maven.version=#{UNKNOWN_MAVEN_VERSION}' in its system.properties file" do
      it "will fail with a descriptive error message" do
        Cutlass::App.new(
          "simple-http-service",
          exception_on_failure: false
        ).transaction do |app|
          set_maven_version(app.tmpdir, UNKNOWN_MAVEN_VERSION)
          app.pack_build do |pack_result|
            expect(pack_result.success?).to be(false)
            expect(pack_result.stderr).to include("[ERROR: Unsupported Maven version]")
            expect(pack_result.stderr).to include("You have defined an unsupported Maven version in the system.properties file.")
            expect(pack_result.stderr).to include("The default supported version is #{DEFAULT_MAVEN_VERSION}")
          end
        end
      end
    end
  end

  context "for an app without Maven wrapper" do
    context "without 'maven.version' in its system.properties file" do
      it "will install Maven #{DEFAULT_MAVEN_VERSION}" do
        Cutlass::App.new("simple-http-service").transaction do |app|
          remove_maven_wrapper(app.tmpdir)

          app.pack_build do |pack_result|
            expect(pack_result.stdout).to include("Selected Maven version: #{DEFAULT_MAVEN_VERSION}")
            expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - MAVEN VERSION] #{DEFAULT_MAVEN_VERSION}")
          end
        end
      end
    end

    context "with 'maven.version=#{UNKNOWN_MAVEN_VERSION}' in its system.properties file" do
      it "will fail with a descriptive error message" do
        Cutlass::App.new(
          "simple-http-service",
          exception_on_failure: false
        ).transaction do |app|
          remove_maven_wrapper(app.tmpdir)
          set_maven_version(app.tmpdir, UNKNOWN_MAVEN_VERSION)

          app.pack_build do |pack_result|
            expect(pack_result.success?).to be(false)
            expect(pack_result.stderr).to include("[ERROR: Unsupported Maven version]")
            expect(pack_result.stderr).to include("You have defined an unsupported Maven version in the system.properties file.")
            expect(pack_result.stderr).to include("The default supported version is #{DEFAULT_MAVEN_VERSION}")
          end
        end
      end
    end

    context "with 'maven.version=3.6.2' in its system.properties file" do
      it "will install Maven 3.6.2" do
        Cutlass::App.new("simple-http-service").transaction do |app|
          remove_maven_wrapper(app.tmpdir)
          set_maven_version(app.tmpdir, "3.6.2")

          app.pack_build do |pack_result|
            expect(pack_result.stdout).to include("Selected Maven version: 3.6.2")
            expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - MAVEN VERSION] 3.6.2")
          end
        end
      end
    end

    context "with 'maven.version=3.5.4' in its system.properties file" do
      it "will install Maven 3.5.4" do
        Cutlass::App.new("simple-http-service").transaction do |app|
          remove_maven_wrapper(app.tmpdir)
          set_maven_version(app.tmpdir, "3.5.4")

          app.pack_build do |pack_result|
            expect(pack_result.stdout).to include("Selected Maven version: 3.5.4")
            expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - MAVEN VERSION] 3.5.4")
          end
        end
      end
    end

    context "with 'maven.version=3.3.9' in its system.properties file" do
      it "will install Maven 3.3.9" do
        Cutlass::App.new("simple-http-service").transaction do |app|
          remove_maven_wrapper(app.tmpdir)
          set_maven_version(app.tmpdir, "3.3.9")

          app.pack_build do |pack_result|
            expect(pack_result.stdout).to include("Selected Maven version: 3.3.9")
            expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - MAVEN VERSION] 3.3.9")
          end
        end
      end
    end

    context "with 'maven.version=3.2.5' in its system.properties file" do
      it "will install Maven 3.2.5" do
        Cutlass::App.new("simple-http-service").transaction do |app|
          remove_maven_wrapper(app.tmpdir)
          set_maven_version(app.tmpdir, "3.2.5")

          app.pack_build do |pack_result|
            expect(pack_result.stdout).to include("Selected Maven version: 3.2.5")
            expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - MAVEN VERSION] 3.2.5")
          end
        end
      end
    end
  end
end
