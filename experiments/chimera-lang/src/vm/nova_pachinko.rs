use crate::vm::ChimeraVM;

pub(crate) fn exec_pachinko(vm: &mut ChimeraVM) {
    vm.output.push("Pachinko physics simulated.".to_string());
}

pub(crate) fn exec_automaton(vm: &mut ChimeraVM) {
    vm.output.push("Automaton logic simulated.".to_string());
}

pub(crate) fn exec_syncopation(vm: &mut ChimeraVM) {
    vm.output.push("Syncopation logic triggered.".to_string());
}

pub(crate) fn exec_choreography(vm: &mut ChimeraVM) {
    vm.output.push("Choreography logic triggered.".to_string());
}

pub(crate) fn exec_runes(vm: &mut ChimeraVM) {
    vm.output.push("Runes logic triggered.".to_string());
}

pub(crate) fn exec_tardis(vm: &mut ChimeraVM) {
    vm.output.push("Tardis logic triggered.".to_string());
}
