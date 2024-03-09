#![allow(unused)]


mod mdm_ap;
pub mod errors;

use mdm_ap::*;
pub use errors::*;

use std::{thread, time};
use std::ops::DerefMut;

use probe_rs::{
    DebugProbeType,
    architecture::arm::{ApAddress, ap::MemoryAp, 
        ArmProbeInterface,
        armv6m::Dhcsr,
        DpAddress},
    DebugProbeInfo,
    MemoryMappedRegister,
    Probe};


pub fn stlink() -> Result<Probe, Error> {
    
    let list = Probe::list_all();
    let device =
    list.iter()
    .filter(|prog: &&DebugProbeInfo |if prog.probe_type == DebugProbeType::StLink { true } else { false } )
    .next()
    .ok_or(Error::MdmExample("StLink not connected - not found by VID & PID".to_string()));
    
    let stlink = device?.open().map_err(|err | Error::MdmExample(format!("StLink Can't Be Open: error {:?}, ",  err )))?;
    Ok(stlink)
}

pub fn debug_mode_on_an4835(mut probe :  Probe) -> Result<(), Error> {
    
    probe.attach_to_unspecified().map_err(|err | Error::MdmExample(format!("Failed base init Programmer. If this happend error after-write erase re-load programmer : error {:?}, ",  err )))?;
    probe.target_reset().map_err(|err | Error::MdmExample(format!("Failed reset target : error {:?}, ",  err)))?;

    let mut iface = probe
       .try_into_arm_interface().map_err(|err | Error::MdmExample(format!("Programmer failed open ARM Interface : error {:?}, ",  err )))?
       .initialize_unspecified()
       .map_err(|_ | Error::MdmExample("Programmer failed init ARM interface".to_string()))?;


    println!("-----------------------------------------------------"); 
    println!("MKE GENERAL INTERFACE : debug_mode_on based on AN4835");
    println!("-----------------------------------------------------"); 

    /*  "SWD connection steps" based on AN4835  */
    /* 1. init mdm ap reg, read current state */
    let mut mdm_ap = MdmAP::read_mdm_ap_register(iface.deref_mut(), false)?;
        
    /* 2. write the System Reset Request bit. Keep reset low and establish communication with the ARM DAP.  */
    mdm_ap.mdm_ap_reset_keep(iface.deref_mut())?;                

    /* 3. The MDM-AP ID register can be read to verify that the connection is working correctly. */                                                                             
    let idr_reg = mdm_ap.read_mdm_ap_idr(iface.deref_mut())?;   
    if idr_reg != IDR_REG_CHECK_VALUE {           
        println!( "IDR_reg != const value 0x001C_0020" );                             
    } else {
        println!( "MKE ID Register {}", &format!("{:#06X} OK ", idr_reg ));
    }

    /* 4. The MDM-AP ID register can be read to verify that the connection is working correctly. */                                                                            
    mdm_ap.is_mdm_flash_ready(iface.deref_mut())?;

    /* 5. Read the System Security bit to determine if security is enabled. If System Security = 0, then proceed. */                                                                             // 
    mdm_ap.refresh_mdm_ap(iface.deref_mut(), false)?;
    if mdm_ap.status.security == true {  
        /* didn't try to halt core, just return Ok, so user can try mass erase chip if decide */
        println!("Target connected, but secured");
        println!("Target is secured, for unsecure mass erase");
        panic!();
    }

    /* 6. Write the MDM-AP register to set the Debug Request bit */
    mdm_ap.write_mdm_ap_control_bit(iface.deref_mut(), MKE_MDM_CONTROL_DBG_REQ_BIT)?;
    //mdm_ap.write_mdm_ap_control_bit(iface.deref_mut(), MKE_MDM_CONTROL_CORE_HOLD_BIT)?;
    mdm_ap.refresh_and_compare_mdm_ap(iface.deref_mut()," DBG_REQ_BIT set ".to_string())?;

    /* Try to write HALT bit on DHCSR reg ARM core */
    // When the steps above have been completed, debugging or flash programming can be started.
    let mut dhcsr = Dhcsr(0);
    dhcsr.set_c_halt(true);
    dhcsr.set_c_debugen(true);
    dhcsr.enable_write();

    let mut probe = iface.memory_interface(MKE_DEFAULT_MEM_AP).map_err(|err | Error::MdmExample(format!("Failed get ARM interface : error {:?}, ",  err)))?;
    let dhcsr_before: u32=   probe.read_word_32(Dhcsr::get_mmio_address()).map_err(|err | Error::MdmExample(format!("Failed read DHCSR ARM reg  : error {:?}, ",  err)))?;
    probe.write_word_32(Dhcsr::get_mmio_address(), dhcsr.into()).map_err(|err | Error::MdmExample(format!("Probe Flush : error {:?}, ",  err)))?;
    probe.flush().map_err(|err | Error::MdmExample(format!("Probe Flush : error {:?}, ",  err)))?;

    thread::sleep(time::Duration::from_millis(500));
    let dhcsr_after = probe.read_word_32(Dhcsr::get_mmio_address()).map_err(|err | Error::MdmExample(format!("Failed check after write DHCSR ARM reg  : error {:?}, ",  err)))?;
    println!("Dhcsr_before write  {:04X}, Dhcsr_after write  {:04X}", &dhcsr_before, &dhcsr_after );
    drop(probe);
    mdm_ap.refresh_mdm_ap(iface.deref_mut(), true)?;

    /*  7. clear the System Reset Request bit in the MDM-AP control register. */
    mdm_ap.mdm_ap_clear_reset_bit(iface.deref_mut()).map_err(|err | Error::MdmExample(format!("Failed mdm_ap_clear_reset_bit  : error {:?}, ",  err)))?;   
    thread::sleep(time::Duration::from_millis(50));
    mdm_ap.refresh_mdm_ap(iface.deref_mut(), true)?;
    
    let mut probe = iface.memory_interface(MKE_DEFAULT_MEM_AP).map_err(|err | Error::MdmExample(format!("Failed get ARM interface : error {:?}, ",  err)))?;
    let dhcsr_end=   probe.read_word_32(Dhcsr::get_mmio_address()).map_err(|err | Error::MdmExample(format!("Failed read DHCSR ARM reg  : error {:?}, ",  err)))?;
    println!("dhcsr_end   {:04X}", &dhcsr_end );
    drop(probe);

    mdm_ap.refresh_mdm_ap(iface.deref_mut(), true)?;

    Ok(())
 
}
pub fn main() {


    let probe_try_stlink = stlink();

    let probe = match probe_try_stlink {
    
        Ok(probe) => { probe }
        Err(err) => { 
            println!("StLink connection error: {:?}", err);
            panic!();
        }
    };

    if let Err(err) = debug_mode_on_an4835(probe) {
        println!("debug_mode_on_an4835 error: {:?}", err);
    }

    
        
}