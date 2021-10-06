require_relative "spec_helper"
require "digest"

describe "Heroku's Java CNB" do
  it "logs" do
    Cutlass::App.new("function_with_logs").transaction do |app|
      app.pack_build do |pack_result|
        expect(pack_result.stdout).to include("Installing Java function runtime")
      end

      app.start_container(expose_ports: 8080) do |container|
        # We must call this twice to trigger a log, IDK why
        query = Cutlass::FunctionQuery.new(
          port: container.get_host_port(8080),
          body: {}
        ).call

        query = Cutlass::FunctionQuery.new(
          port: container.get_host_port(8080),
          body: {}
        ).call

        expect(query.as_json).to eq({ "accounts" => []})
        expect(query.success?).to be_truthy

        expect(container.logs.stdout).to match("logging info 1")
      end
    end
  end

  it "generates a callable salesforce function" do
    Cutlass::App.new("simple-function").transaction do |app|
      app.pack_build do |pack_result|
        expect(pack_result.stdout).to include("Installing Java function runtime")
      end

      app.start_container(expose_ports: 8080) do |container|

        # Calling 2 times doesn't seem to magically fix this example :(
        body = "hello world"
        query = Cutlass::FunctionQuery.new(
          port: container.get_host_port(8080),
          body: body
        ).call

        expect(query.as_json).to eq(body.reverse)
        expect(query.success?).to be_truthy

        # expect(container.logs.stdout).to match("logging info 1")
      end
    end
  end
end
