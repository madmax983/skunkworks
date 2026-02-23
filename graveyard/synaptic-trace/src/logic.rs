use nalgebra::{Isometry3, Translation3, UnitQuaternion, Vector3};
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum SegmentType {
    User,
    System,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TraceSegment {
    pub content: String,
    pub segment_type: SegmentType,
}

pub fn parse_trace(input: &str) -> Vec<TraceSegment> {
    let mut segments = Vec::new();
    let re = Regex::new(r"^\s*\d+:\s*(.*)$").unwrap();
    let sys_prefixes = [
        "std::", "core::", "alloc::", "tokio::", "panic::", "actix::",
    ];

    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let function_name = caps.get(1).map_or("", |m| m.as_str()).trim().to_string();
            let segment_type = if sys_prefixes.iter().any(|&p| function_name.starts_with(p)) {
                SegmentType::System
            } else {
                SegmentType::User
            };
            segments.push(TraceSegment {
                content: function_name,
                segment_type,
            });
        }
    }
    segments
}

pub fn assign_target_angles(segments: &[TraceSegment]) -> Vec<f32> {
    let mut angles = Vec::new();
    let fold_angle = 160.0f32.to_radians();

    // Track current absolute angle to correct back to 0
    let mut current_absolute_angle = 0.0;

    for (i, segment) in segments.iter().enumerate() {
        let angle = match segment.segment_type {
            SegmentType::User => {
                // User segment should try to return to 0 absolute angle relative to the *previous* segment
                let correction = -current_absolute_angle;
                correction
            }
            SegmentType::System => {
                if i == 0 {
                    fold_angle
                } else {
                    let prev_type = &segments[i - 1].segment_type;
                    match prev_type {
                        SegmentType::User => fold_angle,
                        SegmentType::System => {
                            // Toggle direction
                            let last_angle = angles.last().unwrap();
                            if *last_angle > 0.0 {
                                -fold_angle
                            } else {
                                fold_angle
                            }
                        }
                    }
                }
            }
        };

        angles.push(angle);
        current_absolute_angle += angle;
    }

    angles
}

pub fn calculate_strip_transforms(
    segments: &[TraceSegment],
    angles: &[f32],
    progress: f32,
) -> Vec<Isometry3<f32>> {
    let mut transforms = Vec::new();
    let mut current_transform = Isometry3::identity();
    let segment_length = 1.0; // Arbitrary unit

    for (i, _segment) in segments.iter().enumerate() {
        let target_angle = angles[i];
        let current_angle = target_angle * progress;

        // Rotation around X axis
        let rotation = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), current_angle);

        // If not the first segment, translate to the end of the previous one BEFORE rotating
        if i > 0 {
            // Move along Y of the PREVIOUS segment
            let translation = Translation3::new(0.0, segment_length, 0.0);

            // Apply translation then rotation
            current_transform = current_transform * translation * rotation;
        } else {
            current_transform = current_transform * rotation;
        }

        transforms.push(current_transform);
    }

    transforms
}
