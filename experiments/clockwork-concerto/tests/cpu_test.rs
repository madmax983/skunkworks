use bevy::prelude::*;
use clockwork_concerto::cpu::{self, CpuState, Instruction, NoteEvent, Program};
use clockwork_concerto::mechanism::TickEvent;

#[derive(Resource, Default)]
struct NoteCount(usize);

fn count_notes(mut events: EventReader<NoteEvent>, mut count: ResMut<NoteCount>) {
    for _ in events.read() {
        count.0 += 1;
    }
}

#[test]
fn test_cpu_note_event() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(cpu::CpuPlugin);
    app.add_event::<TickEvent>(); // Need to add the event we trigger
    app.init_resource::<NoteCount>();
    app.add_systems(Update, count_notes.after(cpu::cpu_tick_system));

    app.world_mut().spawn(CpuState {
        pc: 0,
        phase: cpu::CpuPhase::Fetch,
        instructions: 0,
        registers: [0; 4],
    });

    // Set program: NOTE(60)
    app.insert_resource(Program(vec![Instruction::Note(60)]));

    // Send TickEvent 1 (Fetch)
    app.world_mut().send_event(TickEvent);
    app.update();

    // Send TickEvent 2 (Decode)
    app.world_mut().send_event(TickEvent);
    app.update();

    // Send TickEvent 3 (Execute)
    app.world_mut().send_event(TickEvent);
    app.update();

    let count = app.world().resource::<NoteCount>().0;
    assert_eq!(count, 1, "Should emit one NoteEvent");
}
