use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub(crate) enum LeiningenBuildpackError {}

#[allow(clippy::too_many_lines)]
pub(crate) fn log_user_errors(_error: LeiningenBuildpackError) {}

impl From<LeiningenBuildpackError> for libcnb::Error<LeiningenBuildpackError> {
    fn from(value: LeiningenBuildpackError) -> Self {
        libcnb::Error::BuildpackError(value)
    }
}
