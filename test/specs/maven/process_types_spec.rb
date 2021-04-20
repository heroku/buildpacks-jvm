require_relative "spec_helper"

describe "Heroku's Maven Cloud Native Buildpack" do
  context "for a Spring Boot app" do
    it "will automatically add a process type for that app" do
      Cutlass::App.new(
        "buildpack-java-spring-boot-test",
        buildpacks: [JVM_BUILDPACK, MAVEN_BUILDPACK], # Note the missing Procfile buildpack in the list of buildpacks
      ).transaction do |app|
        app.pack_build do |pack_result|
          app.start_container(expose_ports: 8080) do |container|
            response = Excon.get("http://localhost:#{container.get_host_port(8080)}/", :idempotent => true, :retry_limit => 5, :retry_interval => 1)
            expect(response.body).to eq("Hello from Spring Boot!")
          end
        end
      end
    end
  end
end
