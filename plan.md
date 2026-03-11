1. **Remove Unsafe Blocks in `experiments/ferrous-ddos/src/physics.rs`**
   - The `ParticleSystem::update` function uses `unsafe` blocks to access particles via raw pointers during a parallel iteration. This is done to read other particles while iterating over them.
   - However, since `self.particles.par_iter()` provides immutable references, we can safely access other elements without `unsafe` if we pass a reference to the slice into the closure or just index the slice safely since it's immutable. Wait, `par_iter()` borrows `self.particles`. Inside the closure, accessing `self.particles` again might not be allowed by the borrow checker if it captures `self`.
   - To fix this cleanly, we can borrow `self.particles.as_slice()` outside the parallel iterator and move that slice reference into the closure. Since `par_iter` borrows the same data immutably, we can also have another immutable reference to the slice.
   - We will replace `unsafe { &*particles_ptr.add(j) }` with `particles_slice[j]`.
2. **Update `.jules/warden.md` with the Threat and Defense**
   - Add a journal entry noting the removal of unsafe raw pointer arithmetic in parallel iterator, mitigating potential UB.
3. **Run Pre-Commit Checks**
   - Call `pre_commit_instructions` and follow steps.
4. **Submit PR**
   - Submit the branch with standard Warden format.
