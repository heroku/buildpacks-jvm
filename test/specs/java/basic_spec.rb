require_relative "spec_helper"
require "digest"

describe "Heroku's Java CNB" do
  it "builds a simple app successfully" do
    rapier.app_dir_from_fixture("simple-http-service") do |app_dir|
      rapier.pack_build(app_dir) do |pack_result|
        expect(pack_result.stdout).to_not include("Installing Maven")
        expect(pack_result.stdout).to include("$ ./mvnw")

        pack_result.start_container(expose_ports: 8080) do |container|
          payload = "Roy_Batty"
          response = Excon.get("http://localhost:#{container.get_host_port(8080)}/?payload=#{payload}", :idempotent => true, :retry_limit => 5, :retry_interval => 1)
          expect(response.body).to eq(Digest::SHA256.hexdigest(payload))
        end
      end
    end
  end
end
