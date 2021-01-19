require_relative "spec_helper"

describe "Heroku's Java CNB" do
  # This test has been disabled for now, it needs to be re-instated as soon as the JDBC feature is added.
  #context "when the DATABASE_URL environment variable is set" do
  #  it "will provide JDBC_DATABASE_URL, JDBC_DATABASE_USERNAME, and JDBC_DATABASE_PASSWORD based on DATABASE_URL during build" do
  #    rapier.app_dir_from_fixture("simple-http-service") do |app_dir|
  #      rapier.pack_build(app_dir, build_env: {:DATABASE_URL => "postgres://AzureDiamond:hunter2@db.example.com:5432/testdb"}) do |pack_result|
  #        expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - JDBC_DATABASE_URL] jdbc:postgresql://db.example.com:5432/testdb?password=hunter2&sslmode=require&user=AzureDiamond")
  #        expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - JDBC_DATABASE_USERNAME] AzureDiamond")
  #        expect(pack_result.stdout).to include("[BUILDPACK INTEGRATION TEST - JDBC_DATABASE_PASSWORD] hunter2")
  #      end
  #    end
  #  end
  #end
end
