import sbt._
import Keys._

object HerokuBuildpackPlugin extends AutoPlugin {
  override lazy val projectSettings = Seq(
    Compile / doc / sources := List(),
    packageDoc / publishArtifact := false,
    packageSrc / publishArtifact := false
  )
}
