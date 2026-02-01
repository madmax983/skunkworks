use ratatui::style::Color;
use ratatui::widgets::canvas::Context;

#[derive(Clone, Debug)]
pub struct Trace {
    pub position: (f64, f64),
    pub content: char,
    pub color: Color,
    pub lifetime: f64,
}

pub struct TraceLayer {
    pub traces: Vec<Trace>,
    pub max_traces: usize,
}

impl TraceLayer {
    pub fn new(max_traces: usize) -> Self {
        Self {
            traces: Vec::new(),
            max_traces,
        }
    }

    pub fn add(&mut self, trace: Trace) {
        if self.traces.len() >= self.max_traces {
            self.traces.remove(0); // Remove oldest
        }
        self.traces.push(trace);
    }

    pub fn update(&mut self) {
        // Decay lifetime
        for trace in &mut self.traces {
            trace.lifetime -= 0.5;
        }
        // Remove dead traces
        self.traces.retain(|t| t.lifetime > 0.0);
    }

    pub fn draw(&self, ctx: &mut Context<'_>) {
        for trace in &self.traces {
            // Determine color based on lifetime
            // As lifetime approaches 0, maybe shift to Gray or DarkGray
            let color = if trace.lifetime < 20.0 {
                Color::DarkGray
            } else {
                trace.color
            };

            ctx.print(
                trace.position.0,
                trace.position.1,
                ratatui::text::Span::styled(
                    trace.content.to_string(),
                    ratatui::style::Style::default().fg(color),
                ),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_layer_add() {
        let mut layer = TraceLayer::new(2);
        layer.add(Trace {
            position: (0.0, 0.0),
            content: 'a',
            color: Color::White,
            lifetime: 10.0,
        });
        layer.add(Trace {
            position: (1.0, 1.0),
            content: 'b',
            color: Color::White,
            lifetime: 10.0,
        });
        assert_eq!(layer.traces.len(), 2);

        // Add one more, should replace first
        layer.add(Trace {
            position: (2.0, 2.0),
            content: 'c',
            color: Color::White,
            lifetime: 10.0,
        });
        assert_eq!(layer.traces.len(), 2);
        assert_eq!(layer.traces[0].content, 'b');
        assert_eq!(layer.traces[1].content, 'c');
    }

    #[test]
    fn test_trace_layer_update() {
        let mut layer = TraceLayer::new(10);
        layer.add(Trace {
            position: (0.0, 0.0),
            content: 'a',
            color: Color::White,
            lifetime: 1.0,
        });

        layer.update(); // lifetime -> 0.5
        assert_eq!(layer.traces.len(), 1);

        layer.update(); // lifetime -> 0.0
        assert_eq!(layer.traces.len(), 0);
    }
}
