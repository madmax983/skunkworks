sed -i 's/let mut model = AudioModel::new(/let mut fallback_model = model; \/\/ Use the existing model/g' experiments/neuro-resonance/src/main.rs
sed -i '/GRID_WIDTH,/d' experiments/neuro-resonance/src/main.rs
sed -i '/GRID_HEIGHT,/d' experiments/neuro-resonance/src/main.rs
sed -i '/bounded(1).1,/d' experiments/neuro-resonance/src/main.rs
sed -i '/bounded(1).0,/d' experiments/neuro-resonance/src/main.rs
sed -i '/None,/d' experiments/neuro-resonance/src/main.rs
sed -i 's/fallback_model.process(&mut buffer);/fallback_model.process(\&mut buffer);/g' experiments/neuro-resonance/src/main.rs
