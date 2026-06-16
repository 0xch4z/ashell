use std::ops::RangeInclusive;

use crate::theme::use_theme;
use iced::{
    Theme,
    widget::{ProgressBar, progress_bar as base},
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ProgressBarHierarchy {
    #[default]
    Primary,
    Secondary,
    Danger,
}

pub fn progress_bar<'a>(
    range: RangeInclusive<f32>,
    value: f32,
    hierarchy: ProgressBarHierarchy,
) -> ProgressBar<'a> {
    let radius = use_theme(|theme| theme.radius);

    base(range, value).style(move |theme: &Theme| {
        let mut style = match hierarchy {
            ProgressBarHierarchy::Primary => base::primary(theme),
            ProgressBarHierarchy::Secondary => base::secondary(theme),
            ProgressBarHierarchy::Danger => base::danger(theme),
        };
        style.border = style.border.rounded(radius.sm);
        style
    })
}
