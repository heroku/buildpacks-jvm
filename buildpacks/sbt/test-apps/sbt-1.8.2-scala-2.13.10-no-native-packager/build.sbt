name := """test-app"""

version := "1.0"

scalaVersion := "2.13.10"

Compile / mainClass := Some("com.heroku.Server")

libraryDependencies ++= Seq(
  "com.twitter" %% "finagle-http" % "22.12.0"
)

lazy val bogusBuildpackTestTask1 = taskKey[Unit]("Bogus task used by the buildpack tests to ensure it's being run")

bogusBuildpackTestTask1 := println("Running bogusBuildpackTestTask1...")

lazy val bogusBuildpackTestTask2 = taskKey[Unit]("Bogus task used by the buildpack tests to ensure it's being run")

bogusBuildpackTestTask2 := println("Running bogusBuildpackTestTask2...")

lazy val bogusBuildpackTestTask3 = taskKey[Unit]("Bogus task used by the buildpack tests to ensure it's being run")

bogusBuildpackTestTask3 := println("Running bogusBuildpackTestTask3...")
