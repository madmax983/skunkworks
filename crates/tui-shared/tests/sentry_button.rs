use ratatui::{buffer::Buffer, layout::Rect, style::Color, widgets::Widget};
use tui_shared::{Button, ButtonState, ButtonStyle};

#[test]
fn test_button_states_and_styles() {
    let area = Rect::new(0, 0, 10, 3);

    struct TestCase {
        style: ButtonStyle,
        state: ButtonState,
        expected_fg: Color,
        expected_bg: Color,
    }

    let test_cases = vec![
        TestCase {
            style: ButtonStyle::Primary,
            state: ButtonState::Normal,
            expected_fg: Color::Black,
            expected_bg: Color::Blue,
        },
        TestCase {
            style: ButtonStyle::Primary,
            state: ButtonState::Hovered,
            expected_fg: Color::Black,
            expected_bg: Color::Cyan,
        },
        TestCase {
            style: ButtonStyle::Primary,
            state: ButtonState::Clicked,
            expected_fg: Color::Blue,
            expected_bg: Color::White,
        },
        TestCase {
            style: ButtonStyle::Primary,
            state: ButtonState::Disabled,
            expected_fg: Color::DarkGray,
            expected_bg: Color::Black,
        },
        TestCase {
            style: ButtonStyle::Secondary,
            state: ButtonState::Normal,
            expected_fg: Color::White,
            expected_bg: Color::DarkGray,
        },
        TestCase {
            style: ButtonStyle::Secondary,
            state: ButtonState::Hovered,
            expected_fg: Color::Black,
            expected_bg: Color::Gray,
        },
        TestCase {
            style: ButtonStyle::Secondary,
            state: ButtonState::Clicked,
            expected_fg: Color::Black,
            expected_bg: Color::White,
        },
        TestCase {
            style: ButtonStyle::Outline,
            state: ButtonState::Normal,
            expected_fg: Color::Gray,
            expected_bg: Color::Reset,
        },
        TestCase {
            style: ButtonStyle::Outline,
            state: ButtonState::Hovered,
            expected_fg: Color::White,
            expected_bg: Color::Reset,
        },
        TestCase {
            style: ButtonStyle::Outline,
            state: ButtonState::Clicked,
            expected_fg: Color::Black,
            expected_bg: Color::White,
        },
        TestCase {
            style: ButtonStyle::Danger,
            state: ButtonState::Normal,
            expected_fg: Color::White,
            expected_bg: Color::Red,
        },
        TestCase {
            style: ButtonStyle::Danger,
            state: ButtonState::Hovered,
            expected_fg: Color::White,
            expected_bg: Color::LightRed,
        },
        TestCase {
            style: ButtonStyle::Danger,
            state: ButtonState::Clicked,
            expected_fg: Color::Red,
            expected_bg: Color::White,
        },
        TestCase {
            style: ButtonStyle::Warning,
            state: ButtonState::Normal,
            expected_fg: Color::Black,
            expected_bg: Color::Yellow,
        },
        TestCase {
            style: ButtonStyle::Warning,
            state: ButtonState::Hovered,
            expected_fg: Color::Black,
            expected_bg: Color::LightYellow,
        },
        TestCase {
            style: ButtonStyle::Warning,
            state: ButtonState::Clicked,
            expected_fg: Color::Yellow,
            expected_bg: Color::Black,
        },
        TestCase {
            style: ButtonStyle::Success,
            state: ButtonState::Normal,
            expected_fg: Color::Black,
            expected_bg: Color::Green,
        },
        TestCase {
            style: ButtonStyle::Success,
            state: ButtonState::Hovered,
            expected_fg: Color::Black,
            expected_bg: Color::LightGreen,
        },
        TestCase {
            style: ButtonStyle::Success,
            state: ButtonState::Clicked,
            expected_fg: Color::Green,
            expected_bg: Color::White,
        },
    ];

    for case in test_cases {
        let button = Button::new("Test")
            .style_variant(case.style)
            .state(case.state);

        let mut buffer = Buffer::empty(area);
        button.render(area, &mut buffer);

        let cell = &buffer[(0, 0)];
        assert_eq!(
            cell.fg, case.expected_fg,
            "Foreground mismatch for {:?} / {:?}",
            case.style, case.state
        );
        assert_eq!(
            cell.bg, case.expected_bg,
            "Background mismatch for {:?} / {:?}",
            case.style, case.state
        );
    }
}
