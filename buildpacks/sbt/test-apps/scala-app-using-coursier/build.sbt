enablePlugins(JavaAppPackaging)

name := """scala-getting-started"""

version := "1.0"

scalaVersion := "2.12.12"

mainClass in Compile := Some("com.example.Server")

libraryDependencies ++= Seq(
  "com.twitter" %% "finagle-http" % "22.12.0"
)
