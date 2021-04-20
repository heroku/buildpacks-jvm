require_relative "spec_helper"

url = "https://gist.githubusercontent.com/Malax/d47323823a3d59249cbb5593c4f1b764/raw/83f196719d2c4d56aec6720964ba7d7c86b71727/download-settings.xml"
url_value = "Main screen turn on."

describe "Heroku's Maven Cloud Native Buildpack" do
  context "when the MAVEN_SETTINGS_URL environment variable is set" do
    it "will download and use the settings.xml form that URL" do
      Cutlass::App.new(
        "simple-http-service",
        config: {MAVEN_SETTINGS_URL: url}
      ).transaction do |app|
        app.pack_build do |pack_result|
          expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - SETTINGS TEST VALUE] #{url_value}")
        end
      end
    end

    it "will fail with a descriptive error message if that settings.xml file could not be downloaded" do
      Cutlass::App.new(
        "simple-http-service",
        config: {MAVEN_SETTINGS_URL: "https://gist.githubusercontent.com/Malax/settings.xml"},
        exception_on_failure: false,
      ).transaction do |app|
        app.pack_build do |pack_result|
          expect(pack_result.success?).to be(false)
          expect(pack_result.stderr).to include("You have set MAVEN_SETTINGS_URL to \"https://gist.githubusercontent.com/Malax/settings.xml\". We tried to download the file at this")
          expect(pack_result.stderr).to include("URL, but the download failed. Please verify that the given URL is correct and try again.")
          # This error message comes from Maven itself. We don't expect Maven to to be executed at all.
          expect(pack_result.stdout).to_not include("[INFO] BUILD FAILURE")
        end
      end
    end
  end

  context "when the MAVEN_SETTINGS_PATH environment variable is set" do
    it "will use that settings.xml file" do
      settings_xml_filename = "forgreatjustice.xml"
      settings_xml_test_value = "Take off every 'ZIG'!!"

      Cutlass::App.new(
        "simple-http-service",
        config: {MAVEN_SETTINGS_PATH: settings_xml_filename},
      ).transaction do |app|
        write_settings_xml(app.tmpdir, settings_xml_filename, settings_xml_test_value)

        app.pack_build do |pack_result|
          expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - SETTINGS TEST VALUE] #{settings_xml_test_value}")
        end
      end
    end
  end

  context "when the MAVEN_SETTINGS_URL and MAVEN_SETTINGS_PATH environment variables are set" do
    it "will give MAVEN_SETTINGS_PATH precedence" do
      settings_xml_filename = "zerowing.xml"
      settings_xml_test_value = "We get signal."

      Cutlass::App.new(
        "simple-http-service",
        config: {MAVEN_SETTINGS_URL: url, MAVEN_SETTINGS_PATH: settings_xml_filename},
      ).transaction do |app|
        write_settings_xml(app.tmpdir, settings_xml_filename, settings_xml_test_value)

        app.pack_build do |pack_result|
          expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - SETTINGS TEST VALUE] #{settings_xml_test_value}")
        end
      end
    end
  end

  context "with an app that has a settings.xml file in the it's root directory" do
    it "will use that settings.xml file" do
      settings_xml_filename = "settings.xml"
      settings_xml_test_value = "Somebody set up us the bomb."
      Cutlass::App.new("simple-http-service").transaction do |app|
        write_settings_xml(app.tmpdir, settings_xml_filename, settings_xml_test_value)

        app.pack_build do |pack_result|
          expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - SETTINGS TEST VALUE] #{settings_xml_test_value}")
        end
      end
    end
  end

  context "with an app that has a settings.xml file in the root directory and the MAVEN_SETTINGS_PATH environment variable is set" do
    it "will give MAVEN_SETTINGS_PATH precedence" do
      zero_wing_filename = "zerowing.xml"
      zero_wing_test_value = "How are you gentlemen !!"
      settings_xml_test_value = "Somebody set up us the bomb."
      Cutlass::App.new(
        "simple-http-service",
        config: {MAVEN_SETTINGS_PATH: zero_wing_filename},
      ).transaction do |app|
        write_settings_xml(app.tmpdir, "settings.xml", settings_xml_test_value)
        write_settings_xml(app.tmpdir, zero_wing_filename, zero_wing_test_value)

        app.pack_build do |pack_result|
          expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - SETTINGS TEST VALUE] #{zero_wing_test_value}")
        end
      end
    end
  end

  context "with an app that has a settings.xml file in the root directory and the MAVEN_SETTINGS_URL environment variable is set" do
    it "will give MAVEN_SETTINGS_URL precedence" do
      Cutlass::App.new(
        "simple-http-service",
        config: {:MAVEN_SETTINGS_URL => url}
      ).transaction do |app|
        settings_xml_test_value = "We get signal."
        write_settings_xml(app.tmpdir, "settings.xml", settings_xml_test_value)
        app.pack_build do |pack_result|
          expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - SETTINGS TEST VALUE] #{url_value}")
        end
      end
    end
  end
end
