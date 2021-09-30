use std::time::Duration;

use serde::Serialize;

use super::TessellationProfile;

#[derive(Debug, Serialize)]
pub struct FlattenedRenderResult {
    pub profile: TessellationProfile,
    pub frame_times: Vec<Duration>,
}
