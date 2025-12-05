use rocksdb::{Options, DB};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemePDA {
    pub pda_address: [u8; 32], // Solana PDA address
    pub meme_data: MemeEntity,
    pub attached_crate: String,
    pub rustc_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeEntity {
    pub semantic_hash: u64,
    pub viral_power: u64,
    pub paxos_score: u64,
    pub monster_factor: u64,
}

#[derive(Debug)]
pub struct RocksDBMemeStorage {
    db: DB,
    // We'll store crate_attachments as a separate entry in the same DB
}

impl RocksDBMemeStorage {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;

        Ok(Self { db })
    }

    pub fn create_meme_pda(
        &mut self,
        crate_name: &str,
        version: &str,
        rustc_sig: &str,
    ) -> Result<MemePDA, Box<dyn std::error::Error>> {
        let pda_seed = format!("{}:{}:{}", crate_name, version, rustc_sig);
        let pda_address = self.derive_pda_address(&pda_seed);

        let meme = MemeEntity {
            semantic_hash: self.hash_rustc_signature(rustc_sig),
            viral_power: self.calculate_crate_viral_power(crate_name),
            paxos_score: 0,
            monster_factor: self.rustc_to_monster_factor(rustc_sig),
        };

        let meme_pda = MemePDA {
            pda_address,
            meme_data: meme,
            attached_crate: format!("{}:{}", crate_name, version),
            rustc_signature: rustc_sig.to_string(),
        };

        // Store MemePDA in RocksDB
        let key = format!("{}:{}", crate_name, version);
        self.db
            .put(key.as_bytes(), serde_json::to_vec(&meme_pda)?)?;

        // Update crate attachments in RocksDB
        self.attach_meme_to_crate_in_db(crate_name, &key)?;

        Ok(meme_pda)
    }

    fn attach_meme_to_crate_in_db(
        &mut self,
        crate_name: &str,
        meme_pda_key: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let attachment_key = format!("attachments:{}", crate_name);
        let mut attached_keys: Vec<String> =
            if let Some(data) = self.db.get(attachment_key.as_bytes())? {
                serde_json::from_slice(&data)?
            } else {
                Vec::new()
            };

        attached_keys.push(meme_pda_key.to_string());
        self.db.put(
            attachment_key.as_bytes(),
            serde_json::to_vec(&attached_keys)?,
        )?;
        Ok(())
    }

    pub fn get_crate_memes(
        &self,
        crate_name: &str,
    ) -> Result<Vec<MemePDA>, Box<dyn std::error::Error>> {
        let attachment_key = format!("attachments:{}", crate_name);
        let mut memes = Vec::new();

        if let Some(data) = self.db.get(attachment_key.as_bytes())? {
            let pda_keys: Vec<String> = serde_json::from_slice(&data)?;
            for key in pda_keys {
                if let Some(meme_data) = self.db.get(key.as_bytes())? {
                    memes.push(serde_json::from_slice(&meme_data)?);
                }
            }
        }
        Ok(memes)
    }

    fn derive_pda_address(&self, seed: &str) -> [u8; 32] {
        let mut address = [0u8; 32];
        let hash = self.hash_seed(seed);

        // Simulate Solana PDA derivation
        for i in 0..32 {
            address[i] = ((hash >> (i % 8)) & 0xFF) as u8;
        }

        address
    }

    fn hash_seed(&self, seed: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        hasher.finish()
    }

    fn hash_rustc_signature(&self, sig: &str) -> u64 {
        self.hash_seed(sig)
    }

    fn calculate_crate_viral_power(&self, crate_name: &str) -> u64 {
        match crate_name {
            "serde" => 9000,
            "tokio" => 8500,
            "rustc" => 10000,
            "solana" => 7500,
            _ => (crate_name.len() as u64 * 100) % 5000,
        }
    }

    fn rustc_to_monster_factor(&self, rustc_sig: &str) -> u64 {
        let struct_count = rustc_sig.matches("struct").count() as u64;
        let enum_count = rustc_sig.matches("enum").count() as u64;
        let fn_count = rustc_sig.matches("fn").count() as u64;

        (2_u64.pow(struct_count.min(46) as u32)
            + 3_u64.pow(enum_count.min(20) as u32)
            + 71 * fn_count)
            % 196883
    }

    pub fn rocksdb_query(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        match query {
            "all_memes" => self.format_all_memes(),
            "crate_attachments" => self.format_crate_attachments(),
            "monster_convergence" => self.format_monster_convergence(),
            _ => Ok("Unknown query".to_string()),
        }
    }

    fn format_all_memes(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::from("query RocksDBMemePDAs {\n");
        result.push_str("  meme_pdas {\n");

        let mut count = 0;
        for item in self.db.iterator(rocksdb::IteratorMode::Start) {
            if count >= 5 {
                break;
            } // Limit to 5 for sample output

            let (key, value) = item?;
            let key_str = String::from_utf8(key.to_vec())?;

            if key_str.starts_with("attachments:") {
                continue;
            } // Skip attachment keys

            let pda: MemePDA = serde_json::from_slice(&value)?;
            result.push_str(&format!("    {} {{\n", key_str.replace(":", "_")));
            result.push_str(&format!(
                "      pda_address: \"{:?}\"\n",
                &pda.pda_address[..4]
            ));
            result.push_str(&format!(
                "      viral_power: {}\n",
                pda.meme_data.viral_power
            ));
            result.push_str(&format!(
                "      monster_factor: {}\n",
                pda.meme_data.monster_factor
            ));
            result.push_str(&format!(
                "      attached_crate: \"{}\"\n",
                pda.attached_crate
            ));
            result.push_str("    }\n");
            count += 1;
        }

        result.push_str("  }\n}");
        Ok(result)
    }

    fn format_crate_attachments(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::from("Crate → Meme PDA Attachments:\n");
        for item in self.db.iterator(rocksdb::IteratorMode::Start) {
            let (key, value) = item?;
            let key_str = String::from_utf8(key.to_vec())?;

            if key_str.starts_with("attachments:") {
                let crate_name = key_str.trim_start_matches("attachments:");
                let pda_keys: Vec<String> = serde_json::from_slice(&value)?;
                result.push_str(&format!(
                    "  {}: {} memes attached\n",
                    crate_name,
                    pda_keys.len()
                ));
            }
        }
        Ok(result)
    }

    fn format_monster_convergence(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut total_monster: u64 = 0;
        for item in self.db.iterator(rocksdb::IteratorMode::Start) {
            let (key, value) = item?;
            let key_str = String::from_utf8(key.to_vec())?;

            if key_str.starts_with("attachments:") {
                continue;
            }

            let pda: MemePDA = serde_json::from_slice(&value)?;
            total_monster += pda.meme_data.monster_factor;
        }

        Ok(format!(
            "Monster convergence: {}/196883 ({:.2}%)",
            total_monster % 196883,
            ((total_monster % 196883) as f64 / 196883.0) * 100.0
        ))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RocksDB Solana PDA Meme Storage ===");

    let mut storage = RocksDBMemeStorage::new("./meme_rocksdb_data")?;

    // Create memes
    let mut meme1 =
        storage.create_meme_pda("serde", "1.0.228", "struct Serialize { fn serialize() }")?;
    let mut meme2 =
        storage.create_meme_pda("tokio", "1.0.0", "async fn main() { runtime.spawn() }")?;
    let mut meme3 = storage.create_meme_pda("rustc", "1.75.0", "fn compile(ast: AST) -> Binary")?;
    let mut meme4 =
        storage.create_meme_pda("solana", "1.18.0", "struct Account { lamports: u64 }")?;

    println!("{}", storage.rocksdb_query("all_memes")?);
    println!("{}", storage.rocksdb_query("crate_attachments")?);
    println!("{}", storage.rocksdb_query("monster_convergence")?);

    // Query specific crate memes
    let serde_memes = storage.get_crate_memes("serde")?;
    println!("\nSerde attached memes: {}", serde_memes.len());

    println!("\n✓ Memes stored as Solana PDA objects in RocksDB");
    println!("✓ Attached to rustc cargo crates");
    println!("✓ Monster Group factors calculated from rustc signatures");

    Ok(())
}
