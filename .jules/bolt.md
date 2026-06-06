**[Optimizing API Return Types Safely]**
**Learning:** Returning `String` for static symbols causes unnecessary heap allocations. Using `Cow<'static, str>` prevents this, but changing an existing `pub fn -> String` breaks the API boundary.
**Action:** Preserve the original `pub fn symbol(&self) -> String` and introduce a non-allocating variant `pub fn symbol_cow(&self) -> Cow<'static, str>`. Have the original method delegate to the new one via `self.symbol_cow().into_owned()`. This achieves zero-cost abstraction internally without breaking downstream callers.
