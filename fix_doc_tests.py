import re

with open('crates/gray-scott/src/lib.rs', 'r') as f:
    content = f.read()

# Replace .unwrap() with ? in doc tests
content = re.sub(
r'''    ///
    /// let mut gs = GrayScott::new\(10, 10\);
    /// let idx = gs\.get_index\(5, 5\)\.unwrap\(\);
    ///
    /// // Directly reduce the concentration of U at the center
    /// gs\.u_mut\(\)\[idx\] = 0\.5;
    /// assert_eq!\(gs\.u\(\)\[idx\], 0\.5\);
    /// ```''',
r'''    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut gs = GrayScott::new(10, 10);
    /// let idx = gs.get_index(5, 5).ok_or("Out of bounds")?;
    ///
    /// // Directly reduce the concentration of U at the center
    /// gs.u_mut()[idx] = 0.5;
    /// assert_eq!(gs.u()[idx], 0.5);
    /// # Ok(())
    /// # }
    /// ```''', content)

content = re.sub(
r'''    ///
    /// let mut gs = GrayScott::new\(10, 10\);
    /// let idx = gs\.get_index\(5, 5\)\.unwrap\(\);
    ///
    /// // Directly add a high concentration of V at the center to trigger a reaction
    /// gs\.v_mut\(\)\[idx\] = 1\.0;
    /// assert_eq!\(gs\.v\(\)\[idx\], 1\.0\);
    /// ```''',
r'''    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut gs = GrayScott::new(10, 10);
    /// let idx = gs.get_index(5, 5).ok_or("Out of bounds")?;
    ///
    /// // Directly add a high concentration of V at the center to trigger a reaction
    /// gs.v_mut()[idx] = 1.0;
    /// assert_eq!(gs.v()[idx], 1.0);
    /// # Ok(())
    /// # }
    /// ```''', content)

content = re.sub(
r'''    /// ```
    /// use gray_scott::GrayScott;
    /// let gs = GrayScott::new\(10, 10\);
    /// let index = gs\.get_index\(5, 5\)\.unwrap\(\);
    /// assert_eq!\(index, 55\);
    /// ```''',
r'''    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use gray_scott::GrayScott;
    /// let gs = GrayScott::new(10, 10);
    /// let index = gs.get_index(5, 5).ok_or("Out of bounds")?;
    /// assert_eq!(index, 55);
    /// # Ok(())
    /// # }
    /// ```''', content)

content = re.sub(
r'''    /// // The V chemical will have diffused and reacted, spreading from the center\.
    /// assert!\(dish\.v\(\)\[dish\.get_index\(10, 10\)\.unwrap\(\)\] > 0\.0\);
    /// ```''',
r'''    /// // The V chemical will have diffused and reacted, spreading from the center.
    /// assert!(dish.v()[dish.get_index(10, 10).ok_or("Out of bounds")?] > 0.0);
    /// # Ok(())
    /// # }
    /// ```''', content)

content = re.sub(
r'''    /// // Simulate "Cell Division" parameters over time\.
    /// let \(feed, kill\) = \(0\.0367, 0\.0649\);
    ///
    /// for _ in 0\.\.10 \{
    ///     dish\.update\(feed, kill, 1\.0\);
    /// \}
    ///
    /// // The V chemical will have diffused and reacted, spreading from the center.
    /// assert!\(dish\.v\(\)\[dish\.get_index\(10, 10\)\.ok_or\("Out of bounds"\)\?\] > 0\.0\);
    /// # Ok\(\(\)\)
    /// # \}
    /// ```''',
r'''    /// // Simulate "Cell Division" parameters over time.
    /// let (feed, kill) = (0.0367, 0.0649);
    ///
    /// for _ in 0..10 {
    ///     dish.update(feed, kill, 1.0);
    /// }
    ///
    /// // The V chemical will have diffused and reacted, spreading from the center.
    /// assert!(dish.v()[dish.get_index(10, 10).ok_or("Out of bounds")?] > 0.0);
    /// # Ok(())
    /// # }
    /// ```''', content)

# update main wrappers if needed
content = re.sub(
r'''    /// // Create a small dish and drop a single "spore" of chemical V in the center.
    /// let mut dish = GrayScott::new\(20, 20\);
    /// dish\.add_chemical\(10, 10, 1\.0\);''',
r'''    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// // Create a small dish and drop a single "spore" of chemical V in the center.
    /// let mut dish = GrayScott::new(20, 20);
    /// dish.add_chemical(10, 10, 1.0);''', content)


with open('crates/gray-scott/src/lib.rs', 'w') as f:
    f.write(content)
