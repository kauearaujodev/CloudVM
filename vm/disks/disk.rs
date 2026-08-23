use std::fs::{self, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

pub struct VirtualDisk {
    pub path: String,
    pub size_gb: u64,
}

impl VirtualDisk {
    pub fn new(
        path: String,
        size_gb: u64,
    ) -> Self {
        Self {
            path,
            size_gb,
        }
    }

    pub fn create(&self) -> Result<(), String> {
        if self.size_gb == 0 {
            return Err(
                "O tamanho do disco deve ser maior que 0 GB."
                    .to_string()
            );
        }

        let path = Path::new(&self.path);

        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .map_err(|e| e.to_string())?;
            }
        }

        let size_bytes =
            self.size_gb
                .checked_mul(1024 * 1024 * 1024)
                .ok_or_else(|| {
                    "Tamanho do disco muito grande."
                        .to_string()
                })?;

        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(path)
            .map_err(|e| e.to_string())?;

        file.set_len(size_bytes)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    pub fn size_bytes(&self) -> u64 {
        self.size_gb * 1024 * 1024 * 1024
    }

    pub fn write(
        &self,
        offset: u64,
        data: &[u8],
    ) -> Result<(), String> {
        let end = offset
            .checked_add(data.len() as u64)
            .ok_or_else(|| {
                "Offset inválido.".to_string()
            })?;

        if end > self.size_bytes() {
            return Err(
                "Tentativa de escrever fora do disco virtual."
                    .to_string()
            );
        }

        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .map_err(|e| e.to_string())?;

        file.seek(SeekFrom::Start(offset))
            .map_err(|e| e.to_string())?;

        file.write_all(data)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn read(
        &self,
        offset: u64,
        size: usize,
    ) -> Result<Vec<u8>, String> {
        let end = offset
            .checked_add(size as u64)
            .ok_or_else(|| {
                "Offset inválido.".to_string()
            })?;

        if end > self.size_bytes() {
            return Err(
                "Tentativa de ler fora do disco virtual."
                    .to_string()
            );
        }

        let mut file = OpenOptions::new()
            .read(true)
            .open(&self.path)
            .map_err(|e| e.to_string())?;

        file.seek(SeekFrom::Start(offset))
            .map_err(|e| e.to_string())?;

        let mut buffer = vec![0u8; size];

        file.read_exact(&mut buffer)
            .map_err(|e| e.to_string())?;

        Ok(buffer)
    }
      }
