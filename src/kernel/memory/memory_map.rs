use crate::kernel::memory::memory_region_type::MemoryType::{Usable,Reserved};
use crate::kernel::memory::memory_region_type::MemoryType;

pub struct MemoryRegion {
    pub start: usize,
    pub end: usize,
    pub kind: MemoryType
}

impl MemoryRegion {

    pub fn constructor(start: usize, end: usize, kind: MemoryType) -> Self {
        Self { start, end, kind }
    }

    pub fn is_usable(&self) -> bool {
        match &self.kind {
            Usable => true,
            Reserved => false
        }
    }

     pub fn size (&self) -> usize {
        return self.end - self.start;
    }
}

pub struct MemoryMap {
    regions: &'static [MemoryRegion]
}

impl MemoryMap { 
    
    pub fn constructor(regions: &'static [MemoryRegion]) -> Self {
        Self { regions }
    }


    pub fn find_regins(&self) -> Option<&MemoryRegion> {
        
        for region_check in self.regions {
            if region_check.is_usable() {
                 return Some(region_check)
            }
        }
        None
    }

    pub fn total_usable_memory(&self) -> usize { 
        let mut total = 0;
        for region_check in self.regions {
            if region_check.is_usable() {
                total += region_check.size();
            }
        }
        total
    }

}



