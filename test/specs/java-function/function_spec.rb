require_relative "spec_helper"
require "digest"

describe "Heroku's Java CNB" do
  it "generates a callable salesforce function" do
    Cutlass::App.new("simple-function").transaction do |app|
      app.pack_build do |pack_result|
        expect(pack_result.stdout).to include("Installing Java function runtime")
      end

      app.start_container(expose_ports: 8080) do |container|
        body = "hello world"
        query = Cutlass::FunctionQuery.new(
          port: container.get_host_port(8080),
          body: body
        ).call

        expect(container.logs.stdout).to include("logging info 1")

        expect(query.as_json).to eq(body.reverse)
        expect(query.success?).to be_truthy
      end
    end
  end
end
