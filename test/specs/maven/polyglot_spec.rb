require_relative "spec_helper"

describe "Heroku's Maven Cloud Native Buildpack" do
  context "with a polyglot Maven app" do
    it "will pass the detect phase and build the app successfully" do
      Cutlass::App.new("simple-http-service-groovy-polyglot").transaction do |app|
        app.pack_build do |pack_result|
          expect(pack_result.stdout).to include("[INFO] BUILD SUCCESS")
        end
      end
    end
  end
end
