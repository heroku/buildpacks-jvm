enablePlugins(JavaAppPackaging)

name := """test-app"""

version := "1.0"

scalaVersion := "2.13.10"

mainClass in Compile := Some("com.heroku.Server")

libraryDependencies ++= Seq(
  "com.twitter" %% "finagle-http" % "22.12.0"
)
