
/*
Effective Address (Dolphin OS)
   80000000  24MB  Main Memory (RAM), write-back cached
   C0000000  24MB  Main Memory (RAM), write-through cached
   C8000000   2MB  Embedded Framebuffer (EFB)
   CC000000        Command Processor (CP)
   CC001000        Pixel Engine (PE)
   CC002000        Video Interface (VI)
   CC003000        Peripheral Interface (PI)
   CC004000        Memory Interface (MI)
   CC005000        DSP and DMA Audio Interface (AID)
   CC006000        DVD Interface (DI)
   CC006400        Serial Interface (SI)
   CC006800        External Interface (EXI)
   CC006C00        Audio Streaming Interface (AIS)
   CC008000        PI FIFO (GX)
   FFF00000   1MB  Boot ROM (first megabyte), used during BS only.


Physical Address (Flipper memory interface)
   00000000  24MB  Main Memory (RAM)
   08000000   2MB  Embedded Framebuffer (EFB)
   0C000000        Command Processor (CP)
   0C001000        Pixel Engine (PE)
   0C002000        Video Interface (VI)
   0C003000        Peripheral Interface (PI)
   0C004000        Memory Interface (MI)
   0C005000        DSP and DMA Audio Interface (AID)
   0C006000        DVD Interface (DI)
   0C006400        Serial Interface (SI)
   0C006800        External Interface (EXI)
   0C006C00        Audio Streaming Interface (AIS)
   0C008000        PI FIFO (GX)
   FFF00000   1MB  Boot ROM (first megabyte)
*/

use super::machine_status::MachineStatus;

const IBAT0U: usize = 528;
const IBAT0L: usize = 529;
const IBAT1U: usize = 530;
const IBAT1L: usize = 531;
const IBAT2U: usize = 532;
const IBAT2L: usize = 533;
const IBAT3U: usize = 534;
const IBAT3L: usize = 535;
const DBAT0U: usize = 536;
const DBAT0L: usize = 537;
const DBAT1U: usize = 538;
const DBAT1L: usize = 539;
const DBAT2U: usize = 540;
const DBAT2L: usize = 541;
const DBAT3U: usize = 542;
const DBAT3L: usize = 543;

pub enum BatType {
   Data,
   Instruction
}

#[derive(Default, Clone, Copy, Debug)]
struct Bat {
   bepi: u16,
   bl: u16,
   vs: bool,
   vp: bool,
   brpn: u16,
   wimg: u8,
   pp: u8
}

#[derive(Debug)]
pub struct Mmu {
   dbat: [Bat; 4],
   ibat: [Bat; 4],
}

impl Mmu {

   pub fn new() -> Mmu {
      Mmu {
         dbat: [Bat::default(); 4],
         ibat: [Bat::default(); 4]
      }
   }

   // generic write to ibat and dbat registers
   pub fn write_bat_reg(&mut self, register: usize, value: u32) {
      match register {
         IBAT0U => self.write_bat_upper(BatType::Instruction, 0, value),
         IBAT1U => self.write_bat_upper(BatType::Instruction, 1, value),
         IBAT2U => self.write_bat_upper(BatType::Instruction, 2, value),
         IBAT3U => self.write_bat_upper(BatType::Instruction, 3, value),
         DBAT0U => self.write_bat_upper(BatType::Data, 0, value),
         DBAT1U => self.write_bat_upper(BatType::Data, 1, value),
         DBAT2U => self.write_bat_upper(BatType::Data, 2, value),
         DBAT3U => self.write_bat_upper(BatType::Data, 3, value),
         IBAT0L => self.write_bat_lower(BatType::Instruction, 0, value),
         IBAT1L => self.write_bat_lower(BatType::Instruction, 1, value),
         IBAT2L => self.write_bat_lower(BatType::Instruction, 2, value),
         IBAT3L => self.write_bat_lower(BatType::Instruction, 3, value),
         DBAT0L => self.write_bat_lower(BatType::Data, 0, value),
         DBAT1L => self.write_bat_lower(BatType::Data, 1, value),
         DBAT2L => self.write_bat_lower(BatType::Data, 2, value),
         DBAT3L => self.write_bat_lower(BatType::Data, 3, value),
         _ => panic!("Invalid bat register")
      }
   }

   fn write_bat_upper(&mut self, bat_type: BatType, index: usize, value: u32) {
      let bat = match bat_type {
         BatType::Data => &mut self.dbat[index],
         BatType::Instruction => &mut self.ibat[index]
      };

      // FixMe: validate BAT value
      // MSRIR | MSRDR = 1
      // (Vs & ~MSRPR) | (Vp & MSRPR) = 1

      bat.bepi = ((value >> 17) & 0b111_1111_1111_1111) as u16;
      bat.bl   = ((value >> 2) & 0b111_1111_1111) as u16;
      bat.vs   = ((value >> 1) & 0b1) != 0;
      bat.vp   = (value & 0b1) != 0;
   }

   fn write_bat_lower(&mut self, bat_type: BatType, index: usize, value:u32) {
      let bat = match bat_type {
         BatType::Data => &mut self.dbat[index],
         BatType::Instruction => &mut self.ibat[index]
      };

      // FixMe: validate BAT value

      bat.brpn = (value >> 17 & 0b111_1111_1111_1111) as u16;
      bat.wimg = (value >> 3 & 0b1_1111) as u8;
      bat.pp   = (value & 0b11) as u8;
   }

   pub fn translate_address(&mut self, bat_type: BatType, msr: &MachineStatus, ea: u32) -> u32 {
      if msr.data_address_translation { // block address translation mode (BAT)

         let bats = match bat_type {
            BatType::Data => &self.dbat,
            BatType::Instruction => &self.ibat
         };

         for x in 0..bats.len() {
            let bat = &bats[x];

            let ea_15   = (ea >> 17) as u16;
            let ea_bepi = (ea_15 & 0x7800) ^ ((ea_15 & 0x7FF) & (!bat.bl));

            if ea_bepi == bat.bepi {

               if (!msr.privilege_level && bat.vs) || (msr.privilege_level && bat.vp) {
                  let upper = (bat.brpn ^ ((ea_15 & 0x7FF) & bat.bl)) as u32;
                  let lower = (ea & 0x1FFFF) as u32;

                  return (upper << 17) ^ lower;
               }

            }
         }

         panic!("FixMe: perform address translation with Segment Register");

      } else { // read address translation mode
         ea
      }
   }
}
