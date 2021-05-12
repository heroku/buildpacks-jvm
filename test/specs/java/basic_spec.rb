require_relative "spec_helper"
require "digest"

describe "Heroku's Java CNB" do
  it "builds a simple app successfully" do
    Cutlass::App.new("simple-http-service").transaction do |app|
      app.pack_build do |pack_result|
        expect(pack_result.stdout).to_not include("Downloading and extracting Maven tarball...")
        expect(pack_result.stdout).to include("[INFO] $ ./mvnw")
      end

      app.start_container(expose_ports: 8080) do |container|
        payload = "Roy_Batty"
        response = Excon.get("http://localhost:#{container.get_host_port(8080)}/?payload=#{payload}", :idempotent => true, :retry_limit => 5, :retry_interval => 1)
        expect(response.body).to eq(Digest::SHA256.hexdigest(payload))
      end
    end
  end
end
