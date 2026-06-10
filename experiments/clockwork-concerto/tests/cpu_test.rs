use bevy::prelude::*;
use clockwork_concerto::*;

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
    app.add_plugins(CpuPlugin);
    app.add_event::<TickEvent>(); // Need to add the event we trigger
    app.init_resource::<NoteCount>();
    app.add_systems(Update, count_notes.after(cpu_tick_system));

    app.world.spawn(CpuState {
        pc: 0,
        phase: CpuPhase::Fetch,
        instructions: 0,
        registers: [0; 4],
    });

    // Set program: NOTE(60)
    app.insert_resource(Program(vec![Instruction::Note(60)]));

    // Send TickEvent 1 (Fetch)
    app.world.send_event(TickEvent);
    app.update();

    // Send TickEvent 2 (Decode)
    app.world.send_event(TickEvent);
    app.update();

    // Send TickEvent 3 (Execute)
    app.world.send_event(TickEvent);
    app.update();

    let count = app.world.resource::<NoteCount>().0;
    assert_eq!(count, 1, "Should emit one NoteEvent");
}
