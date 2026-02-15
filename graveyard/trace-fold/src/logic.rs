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
                // Wait, relative angle is what we store.
                // If previous absolute was A, we want new absolute to be 0.
                // So relative = 0 - A.
                let correction = -current_absolute_angle;
                // Normalize to [-pi, pi]
                // (Optional, but good for stability)
                correction
            }
            SegmentType::System => {
                // Accordion pattern
                // If previous was User, start +
                // If previous was System, flip sign of previous relative angle?
                // Simpler: use index parity or a state tracker.

                // Let's look at the previous angle.
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

    // Base of the first segment is at 0,0,0
    // But we want the transforms of the *centers* or *bases*?
    // Usually convenient to return the base transform of each segment.

    for (i, _segment) in segments.iter().enumerate() {
        // The hinge is at the START of the segment (relative to previous end)
        // Except for the first segment, which is grounded.

        let target_angle = angles[i];
        let current_angle = target_angle * progress;

        // Rotation around X axis
        let rotation = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), current_angle);

        // If not the first segment, translate to the end of the previous one BEFORE rotating
        if i > 0 {
            // Move along Y of the PREVIOUS segment
            let translation = Translation3::new(0.0, segment_length, 0.0);

            // Apply translation then rotation
            // T_current = T_prev * Translate * Rotate
            current_transform = current_transform * translation * rotation;
        } else {
            // First segment just rotates at origin (or maybe stays fixed?)
            // Let's say first segment is fixed. Angle applies to the JOINT.
            // If i=0, that's the joint before the first segment?
            // Let's say angles[i] is the angle between seg[i-1] and seg[i].
            // For i=0, it's angle relative to world Y.
            current_transform = current_transform * rotation;
        }

        transforms.push(current_transform);
    }

    transforms
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point3;

    #[test]
    fn test_parse_simple_trace() {
        let input = "
stack backtrace:
   0: std::backtrace_rs::backtrace::libunwind::trace
             at /rustc/std/src/lib.rs:100
   1: core::fmt::num::imp::fmt_u64
             at /rustc/core/src/fmt/num.rs:200
   2: my_app::main
             at src/main.rs:10
";
        let segments = parse_trace(input);

        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].segment_type, SegmentType::System);
        assert_eq!(segments[1].segment_type, SegmentType::System);
        assert_eq!(segments[2].segment_type, SegmentType::User);
    }

    #[test]
    fn test_angles_accordion() {
        let segments = vec![
            TraceSegment {
                content: "s1".into(),
                segment_type: SegmentType::System,
            },
            TraceSegment {
                content: "s2".into(),
                segment_type: SegmentType::System,
            },
            TraceSegment {
                content: "s3".into(),
                segment_type: SegmentType::System,
            },
            TraceSegment {
                content: "u1".into(),
                segment_type: SegmentType::User,
            },
        ];

        let angles = assign_target_angles(&segments);

        // System start
        assert!(angles[0] > 0.0);
        // System toggle
        assert!(angles[1] < 0.0);
        // System toggle
        assert!(angles[2] > 0.0);

        // User correction:
        // Sum so far: +A -A +A = +A.
        // Correction should be -A.
        assert!((angles[3] + angles[0] + angles[1] + angles[2]).abs() < 0.001);
    }

    #[test]
    fn test_folding_compresses() {
        let segments = vec![
            TraceSegment {
                content: "s1".into(),
                segment_type: SegmentType::System,
            },
            TraceSegment {
                content: "s2".into(),
                segment_type: SegmentType::System,
            },
        ];
        let angles = assign_target_angles(&segments); // +160, -160

        // Unfolded (progress 0)
        let t_unfolded = calculate_strip_transforms(&segments, &angles, 0.0);

        // Folded (progress 1)
        let t_folded = calculate_strip_transforms(&segments, &angles, 1.0);

        let tip_unfolded = t_unfolded[1] * Point3::new(0.0, 1.0, 0.0);
        let tip_folded = t_folded[1] * Point3::new(0.0, 1.0, 0.0);

        assert!(tip_folded.coords.magnitude() < tip_unfolded.coords.magnitude());
    }
}
