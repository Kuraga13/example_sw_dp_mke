use super::*;

/// `MKE_MDM_AP_PORT` - AccessPort to `miscellaneous debug module (MDM)`
pub const MKE_DEFAULT_MEM_AP: MemoryAp = MemoryAp::new(ApAddress {
    dp: DpAddress::Default,
    ap: 0,
});

/// `MKE_MDM_AP_PORT` - AccessPort to `miscellaneous debug module (MDM)`
pub const MKE_MDM_AP_PORT: ApAddress = ApAddress {
    dp: DpAddress::Default,
    ap: 1,
};

/// `MKE_MDM_STATUS`
/// `miscellaneous debug module (MDM)` is implemented on this device, which contains
/// the DAP control and status registers
///
/// packet is u32
///
/// MDM-AP SELECT[31:24] = 0x01 selects the MDM-AP
///
/// SELECT[7:4] = 0x0 selects the bank with Status and Ctrl
///
/// A[3:2] = 2’b00 selects the Status Register
///
/// A[3:2] = 2’b01 selects the Control Register
///
/// SELECT[7:4] = 0xF selects the bank with IDR
///
/// A[3:2] = 2’b11 selects the IDR Register
pub const MKE_MDM_STATUS: u8 = 0x0;
pub const MKE_MDM_STATUS_FLASH_MASS_ERASE_ACK_BIT: u32 = 0b00000001;
pub const MKE_MDM_STATUS_FLASH_READY_BIT: u32 = 0b00000010;
pub const MKE_MDM_STATUS_SYSTEM_SECURITY_BIT: u32 = 0b00000100;
pub const MKE_MDM_STATUS_SYSTEM_RESET_BIT: u32 = 0b00001000;

/// `MKE_MDM_CONTROL`
/// `miscellaneous debug module (MDM)` is implemented on this device, which contains
/// the DAP control and status registers
///
/// packet is u32
///
/// MDM-AP SELECT[31:24] = 0x01 selects the MDM-AP
///
/// SELECT[7:4] = 0x0 selects the bank with Status and Ctrl
///
/// A[3:2] = 2’b00 selects the Status Register
///
/// A[3:2] = 2’b01 selects the Control Register
///
/// SELECT[7:4] = 0xF selects the bank with IDR
///
/// A[3:2] = 2’b11 selects the IDR Register
pub const MKE_MDM_CONTROL: u8 = 0x04;
pub const MKE_MDM_CONTROL_FLASH_MASS_ERASE_BIT: u32 = 0b00000001;
pub const MKE_MDM_CONTROL_DBG_DIS_BIT: u32 = 0b00000010;
pub const MKE_MDM_CONTROL_DBG_REQ_BIT: u32 = 0b00000100;
pub const MKE_MDM_CONTROL_SYS_RESET_BIT: u32 = 0b00001000;
pub const MKE_MDM_CONTROL_CORE_HOLD_BIT: u32 = 0b00010000;

///`MKE_MDM_IDR_REG` = IDR register always reads `0x001C_0020`
///
/// `miscellaneous debug module (MDM)` is implemented on this device, which contains
/// the DAP control and status registers
///
/// MDM-AP SELECT`[31:24]` = 0x01 selects the MDM-AP
///
/// `SELECT[7:4]` = 0xF selects the bank with IDR
///
/// `A[3:2]` = 2’b11 selects the IDR Register
pub const MKE_MDM_IDR_REG: u8 = 0xFC;

pub const IDR_REG_CHECK_VALUE: u32 = 0x001C_0020;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct MdmApStatus {
    ///`Flash Mass Erase Acknowledge`
    /// The  field is cleared after POR reset. The field is also cleared at launch of a mass erase command due to write of
    /// Flash Mass Erase in Progress field in MDM AP Control Register. The
    /// Flash Mass Erase Acknowledge is set after Flash control logic has started
    /// the mass erase operation.
    mass_erase_ack: bool,
    flash_ready: bool,  // Flash is ready?
    pub security: bool, // device is secured?
    system_reset: bool, // 0 = system in reset, 1 = not in reset
    halt_state: bool,   // core halted ?
    stop_state: bool,   // stop mode ?
    wait_state: bool,   // wait mode ?
    value: u32,
}

impl MdmApStatus {
    pub fn parse_from_u32(status_lword: u32) -> Self {
        let status_byte0 = ((status_lword >> 0) & 0xFF) as u8;

        let status_byte2 = ((status_lword >> 16) & 0xFF) as u8;

        let mut mdm_status = MdmApStatus::default();

        mdm_status.value = status_lword;

        if status_byte0 & 0b0000_0001 != 0 {
            mdm_status.mass_erase_ack = true
        }

        if status_byte0 & 0b0000_0010 != 0 {
            mdm_status.flash_ready = true
        }

        if status_byte0 & 0b0000_0100 != 0 {
            mdm_status.security = true
        }

        if status_byte0 & 0b0000_1000 != 0 {
            mdm_status.system_reset = true
        }

        if status_byte2 & 0b0000_0001 != 0 {
            mdm_status.halt_state = true
        }

        if status_byte2 & 0b0000_0010 != 0 {
            mdm_status.stop_state = true
        }

        if status_byte2 & 0b0000_0100 != 0 {
            mdm_status.wait_state = true
        }

        mdm_status
    }

    fn compare(&self, other: &MdmApStatus) {
        let mut no_changes = true;
        println!("---------------- compare status changes ----------------");
        if self.mass_erase_ack != other.mass_erase_ack {
            println!(
                "mass_erase_ack changed from {} to {}",
                self.mass_erase_ack, other.mass_erase_ack
            );
            no_changes = false;
        }
        if self.flash_ready != other.flash_ready {
            println!(
                "flash_ready changed from {} to {}",
                self.flash_ready, other.flash_ready
            );
            no_changes = false;
        }
        if self.security != other.security {
            println!(
                "security changed from {} to {}",
                self.security, other.security
            );
            no_changes = false;
        }
        if self.system_reset != other.system_reset {
            println!(
                "system_reset changed from {} to {}",
                self.system_reset, other.system_reset
            );
            no_changes = false;
        }
        if self.halt_state != other.halt_state {
            println!(
                "halt_state changed from {} to {}",
                self.halt_state, other.halt_state
            );
            no_changes = false;
        }
        if self.stop_state != other.stop_state {
            println!(
                "stop_state changed from {} to {}",
                self.stop_state, other.stop_state
            );
            no_changes = false;
        }
        if self.wait_state != other.wait_state {
            println!(
                "wait_state changed from {} to {}",
                self.wait_state, other.wait_state
            );
            no_changes = false;
        }
        if no_changes == true {
            println!("mdm_ap_status : no changes");
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct MdmApControl {
    ///`erase_in_progress` Set to cause mass erase. Cleared by hardware after mass erase operation completes.
    erase_in_progress: bool,
    ///`debug_disable` forces to disable Debug logic.                                
    debug_disable: bool,
    ///`debug_request` force enable debug
    debug_request: bool,
    /// `sys_reset_request` force reset
    sys_reset_request: bool,
    /// `core_hold` used to control exit from sys reset, if set-release with reset and here 1 - core will be start
    /// first, before all system. if set-release reset and here 0 - all sys reset actions/
    /// core will be init after reset after all system init
    core_hold: bool,
    value: u32,
}

impl MdmApControl {
    pub fn parse_from_u32(control_lword: u32) -> Self {
        let control_byte0 = ((control_lword >> 0) & 0xFF) as u8;

        let mut mdm_control = MdmApControl::default();

        mdm_control.value = control_lword;

        if control_byte0 & 0b0000_0001 != 0 {
            mdm_control.erase_in_progress = true
        }

        if control_byte0 & 0b0000_0010 != 0 {
            mdm_control.debug_disable = true
        }

        if control_byte0 & 0b0000_0100 != 0 {
            mdm_control.debug_request = true
        }

        if control_byte0 & 0b0000_1000 != 0 {
            mdm_control.sys_reset_request = true
        }

        if control_byte0 & 0b0001_0000 != 0 {
            mdm_control.core_hold = true
        }

        mdm_control
    }

    fn compare(&self, other: &MdmApControl) {
        println!("---------------- compare control changes ----------------");
        let mut no_changes = true;
        if self.erase_in_progress != other.erase_in_progress {
            println!(
                "erase_in_progress changed from {} to {}",
                self.erase_in_progress, other.erase_in_progress
            );
            no_changes = false;
        }
        if self.debug_disable != other.debug_disable {
            println!(
                "debug_disable changed from {} to {}",
                self.debug_disable, other.debug_disable
            );
            no_changes = false;
        }
        if self.debug_request != other.debug_request {
            println!(
                "debug_request changed from {} to {}",
                self.debug_request, other.debug_request
            );
            no_changes = false;
        }
        if self.sys_reset_request != other.sys_reset_request {
            println!(
                "sys_reset_request changed from {} to {}",
                self.sys_reset_request, other.sys_reset_request
            );
            no_changes = false;
        }
        if self.core_hold != other.core_hold {
            println!(
                "core_hold changed from {} to {}",
                self.core_hold, other.core_hold
            );
            no_changes = false;
        }
        if no_changes == true {
            println!("mdm_ap_control : no changes");
        }
    }
}

///`MKE MDM_AP`
/// `miscellaneous debug module (MDM)` is implemented on this device, which contains
/// the DAP control and status registers
///
/// packet is u32
///
/// MDM-AP SELECT[31:24] = 0x01 selects the MDM-AP
/// SELECT[7:4] = 0x0 selects the bank with Status and Ctrl
/// A[3:2] = 2’b00 selects the Status Register
/// A[3:2] = 2’b01 selects the Control Register
/// SELECT[7:4] = 0xF selects the bank with IDR
/// A[3:2] = 2’b11 selects the IDR Register
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct MdmAP {
    pub status: MdmApStatus,
    pub control: MdmApControl,
}

impl MdmAP {
    /// `read_mdm_ap_register` - get two longword of MDM Status & Control register
    ///  Return packed-struct (bits-field) `MdmAP` with currently status of MDM
    pub fn read_mdm_ap_register(
        iface: &mut dyn ArmProbeInterface,
        print: bool,
    ) -> Result<Self, Error> {
        let mdm_ap_status = iface
            .read_raw_ap_register(MKE_MDM_AP_PORT, MKE_MDM_STATUS)
            .map_err(|err| Error::MdmExample("iface.read_raw_ap_register".to_string()))?;
        let mdm_ap_control = iface
            .read_raw_ap_register(MKE_MDM_AP_PORT, MKE_MDM_CONTROL)
            .map_err(|err| Error::MdmExample("iface.read_raw_ap_register".to_string()))?;

        if (print) {
            println!(
                "mdm_ap_status  {:04X}, mdm_ap_control  {:04X}",
                &mdm_ap_status, &mdm_ap_control
            );
        }

        let status = MdmApStatus::parse_from_u32(mdm_ap_status);
        let control = MdmApControl::parse_from_u32(mdm_ap_control);

        let mdm_ap = Self { status, control };

        Ok(mdm_ap)
    }

    /// `write_mdm_ap_control_bit` - write only one bit, hold another without changes
    ///  Return packed-struct (bits-field) `MdmAP` with currently status of MDM
    pub fn write_mdm_ap_control_bit(
        &mut self,
        iface: &mut dyn ArmProbeInterface,
        bit: u32,
    ) -> Result<(), Error> {
        let change_one_bit: u32 = self.control.value | bit;
        iface
            .write_raw_ap_register(MKE_MDM_AP_PORT, MKE_MDM_CONTROL, change_one_bit)
            .map_err(|err| {
                Error::MdmExample(format!(
                    "While write_mdm_ap_control_bit {:#06X}, old_value {:#06X}, : error {:?}, ",
                    bit, self.control.value, err
                ))
            })?;
        Ok(())
    }

    /// `write_mdm_ap_control_clear_bit` - clear one bit
    pub fn write_mdm_ap_control_clear_bit(
        &mut self,
        iface: &mut dyn ArmProbeInterface,
        bit: u32,
    ) -> Result<(), Error> {
        let clear_one_bit: u32 = self.control.value & !bit;
        iface
            .write_raw_ap_register(MKE_MDM_AP_PORT, MKE_MDM_CONTROL, clear_one_bit)
            .map_err(|err| {
                Error::MdmExample(format!(
                    "While write_mdm_ap_control_bit {:#06X}, old_value {:#06X}, : error {:?}, ",
                    bit, self.control.value, err
                ))
            })?;
        Ok(())
    }

    /// `write_mdm_ap_control_new` - write new value to mdm_ap
    pub fn write_mdm_ap_control_new(
        &mut self,
        iface: &mut dyn ArmProbeInterface,
        value: u32,
    ) -> Result<(), Error> {
        iface
            .write_raw_ap_register(MKE_MDM_AP_PORT, MKE_MDM_CONTROL, value)
            .map_err(|err| {
                Error::MdmExample(format!(
                    "While write_mdm_ap_control_new {:#06X}, old_value {:#06X}, : error {:?}, ",
                    value, self.control.value, err
                ))
            })?;
        Ok(())
    }

    pub fn compare(&self, updated: &MdmAP) {
        self.status.compare(&updated.status);
        self.control.compare(&updated.control);
    }

    pub fn read_mdm_ap_idr(&self, iface: &mut dyn ArmProbeInterface) -> Result<u32, Error> {
        let idr = iface
            .read_raw_ap_register(MKE_MDM_AP_PORT, MKE_MDM_IDR_REG)
            .map_err(|err| Error::MdmExample("iface.read_raw_ap_register".to_string()))?;
        Ok(idr)
    }

    pub fn print(&self) {
        println!("----------------MDM_AP_STATUS----------------");
        println!("mass_erase_ack is {:?}", &self.status.mass_erase_ack);
        println!("flash_ready is {:?}", &self.status.flash_ready);
        println!("security is {:?}", &self.status.security);
        println!("system_reset is {:?}", &self.status.system_reset);
        println!("halt_state is {:?}", &self.status.halt_state);
        println!("stop_state is {:?}", &self.status.stop_state);
        println!("wait_state is {:?}", &self.status.wait_state);
        println!("--------------------------------------------");

        println!("----------------MDM_AP_CONTROL----------------");
        println!("erase_in_progress is {:?}", &self.control.erase_in_progress);
        println!("debug_disable is {:?}", &self.control.debug_disable);
        println!("debug_request is {:?}", &self.control.debug_request);
        println!("sys_reset_request is {:?}", &self.control.sys_reset_request);
        println!("core_hold is {:?}", &self.control.core_hold);
        println!("--------------------------------------------");
    }

    /// `refresh_mdm_ap` - read & store `MdmAP` in MKExxZZ
    pub fn refresh_mdm_ap(
        &mut self,
        mut iface: &mut dyn ArmProbeInterface,
        print: bool,
    ) -> Result<Self, Error> {
        let mut new_mdm_ap = MdmAP::read_mdm_ap_register(iface.deref_mut(), print)?;

        if (print) {
            new_mdm_ap.print();
        }

        self.status = new_mdm_ap.status;
        self.control = new_mdm_ap.control;

        Ok(new_mdm_ap)
    }

    pub fn refresh_and_compare_mdm_ap(
        &mut self,
        mut iface: &mut dyn ArmProbeInterface,
        track_reason: String,
    ) -> Result<Self, Error> {
        let updated_mdm_ap = MdmAP::read_mdm_ap_register(iface.deref_mut(), true)?;

        println!("{}", &track_reason);

        self.compare(&updated_mdm_ap);

        self.status = updated_mdm_ap.status;
        self.control = updated_mdm_ap.control;

        Ok(updated_mdm_ap)
    }

    

    pub fn mdm_ap_reset_keep(
        &mut self,
        mut iface: &mut dyn ArmProbeInterface,
    ) -> Result<(), Error> {
        self.write_mdm_ap_control_new(iface.deref_mut(), MKE_MDM_CONTROL_SYS_RESET_BIT)?;
        let mut system_is_reset: bool = false;
        for retry in 0..20 {
            thread::sleep(time::Duration::from_millis(1));
            self.refresh_mdm_ap(iface.deref_mut(), false)?;
            if (self.status.value & MKE_MDM_STATUS_SYSTEM_RESET_BIT == 0) {
                println!(
                    " Reset: MPM_AP.MKE_MDM_STATUS_SYSTEM_RESET_BIT = 0 (System is IN reset) "
                );
                system_is_reset = true;
                break;
            }
        }
        if (!system_is_reset) {
            return Err(Error::MdmExample(
                " Reset: System not reseting reset after 20ms".into(),
            ));
        }

        Ok(())
    }


    pub fn mdm_ap_clear_reset_bit(
        &mut self,
        mut iface: &mut dyn ArmProbeInterface,
    ) -> Result<(), Error> {
        self.write_mdm_ap_control_clear_bit(iface.deref_mut(), MKE_MDM_CONTROL_SYS_RESET_BIT)?;
        Ok(())
    }

    pub fn is_mdm_flash_ready(
        &mut self,
        mut iface: &mut dyn ArmProbeInterface,
    ) -> Result<(), Error> {
        self.refresh_mdm_ap(iface.deref_mut(), true)?;
        println!(" Waiting mdm_ap_flash_ready_bit ");
        let mut flash_ready = false;
        for retry in 0..20 {
            self.refresh_mdm_ap(iface.deref_mut(), false)?;
            if (self.status.value & MKE_MDM_STATUS_FLASH_READY_BIT != 0) {
                flash_ready = true;
                println!(" MPM_AP.flash_ready_bit = 1 (Flash is ready) ");
                break;
            }
        }
        self.refresh_mdm_ap(iface.deref_mut(), true)?;
        if (!flash_ready) {
            return Err(Error::MdmExample(
                "Flash module not ready! MKE_MDM_STATUS_FLASH_READY_BIT = 1".into(),
            ));
        }

        Ok(())
    }

    
}
