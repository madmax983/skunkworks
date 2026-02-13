use reed_solomon_erasure::galois_8::ReedSolomon;
use crc32fast::Hasher;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Sarcophagus {
    pub shards: Vec<Vec<u8>>,
    pub checksums: Vec<u32>,
    pub data_shards_count: usize,
    pub parity_shards_count: usize,
    pub shard_size: usize,
    pub original_len: usize,
    encoder: ReedSolomon,
}

impl Sarcophagus {
    pub fn new(data: &[u8], data_shards_count: usize, parity_shards_count: usize) -> Self {
        let encoder = ReedSolomon::new(data_shards_count, parity_shards_count).unwrap();
        let original_len = data.len();

        // Calculate shard size (ceiling division)
        // Ensure shard size is > 0
        let shard_size = if data_shards_count > 0 {
             (original_len + data_shards_count - 1) / data_shards_count
        } else {
            0
        };

        let mut shards = vec![vec![0u8; shard_size]; data_shards_count + parity_shards_count];

        // Fill data shards
        for (i, chunk) in data.chunks(shard_size).enumerate() {
            if i < data_shards_count {
                shards[i][..chunk.len()].copy_from_slice(chunk);
            }
        }

        // Compute parity
        encoder.encode(&mut shards).unwrap();

        // Compute initial checksums
        let checksums = shards.iter().map(|s| crc32(s)).collect();

        Self {
            shards,
            checksums,
            data_shards_count,
            parity_shards_count,
            shard_size,
            original_len,
            encoder,
        }
    }

    pub fn corrupt(&mut self, shard_idx: usize, byte_idx: usize, value: u8) {
        if shard_idx < self.shards.len() && byte_idx < self.shard_size {
            self.shards[shard_idx][byte_idx] = value;
        }
    }

    // Corrupt a range (burst error)
    #[allow(dead_code)]
    pub fn corrupt_range(&mut self, shard_idx: usize, start: usize, len: usize, value: u8) {
         if shard_idx < self.shards.len() {
             let end = (start + len).min(self.shard_size);
             for i in start..end {
                 self.shards[shard_idx][i] = value;
             }
         }
    }

    pub fn reconstruct(&self) -> Result<Vec<u8>, String> {
        let mut shards_copy: Vec<Option<Vec<u8>>> = Vec::with_capacity(self.shards.len());
        let mut valid_count = 0;

        for (i, shard) in self.shards.iter().enumerate() {
            let current_crc = crc32(shard);
            if current_crc == self.checksums[i] {
                shards_copy.push(Some(shard.clone()));
                valid_count += 1;
            } else {
                shards_copy.push(None);
            }
        }

        if valid_count < self.data_shards_count {
            return Err("Not enough valid shards to reconstruct".to_string());
        }

        let mut result_shards = shards_copy;
        match self.encoder.reconstruct(&mut result_shards) {
            Ok(_) => {
                // Combine data shards
                let mut data = Vec::with_capacity(self.original_len);
                for i in 0..self.data_shards_count {
                     if let Some(shard) = &result_shards[i] {
                         data.extend_from_slice(shard);
                     } else {
                         return Err("Failed to reconstruct data shard".to_string());
                     }
                }
                // Truncate can panic if len > capacity, but extend_from_slice ensures capacity grows?
                // Actually truncate just sets len.
                if data.len() >= self.original_len {
                    data.truncate(self.original_len);
                }
                Ok(data)
            }
            Err(e) => Err(format!("Reconstruction failed: {:?}", e)),
        }
    }
}

fn crc32(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recovery_ok() {
        let data = b"Hello World! This is a test of the Reed-Solomon codec.";
        let mut sarc = Sarcophagus::new(data, 4, 2);

        // Corrupt 1 shard (less than parity 2)
        sarc.corrupt(0, 0, 0xFF);

        let recovered = sarc.reconstruct().expect("Should recover");
        assert_eq!(recovered, data);
    }

    #[test]
    fn test_recovery_fail() {
        let data = b"Hello World! This is a test of the Reed-Solomon codec.";
        let mut sarc = Sarcophagus::new(data, 4, 2);

        // Corrupt 3 shards (more than parity 2)
        sarc.corrupt(0, 0, 0xFF);
        sarc.corrupt(1, 0, 0xFF);
        sarc.corrupt(2, 0, 0xFF);

        let result = sarc.reconstruct();
        assert!(result.is_err());
    }
}
