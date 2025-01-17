use raw_cpuid::CpuId;
use crate::println;

pub fn show_cpu_info() {
    let cpuid = CpuId::new();
    println!("CPU Vendor: {}", cpuid.get_vendor_info().unwrap().as_str());
    println!("CPU Model Name: {}", cpuid.get_processor_brand_string().unwrap().as_str());
}
