extern crate sysinfo;
use sysinfo::*;

struct CpuInfo {
    name:           &'static str,
    count:          usize,
    frequency:      usize,
    thread_count:   usize,
}

impl CpuInfo {
    fn new() {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
    
        let available_memory = system.available_memory();
    
        let logical_cpu = system.cpus().len();
        let cpu_name = system.cpus()[0].brand();
        let cpu_frequency = system.cpus()[0].frequency();
        
        info!("Процессор: {}", cpu_name);
        info!("Количество логических процессоров: {}", logical_cpu);
        info!("Частота процессора: {} Ггц", cpu_frequency);
        info!("Доступно оперативной памяти: {} мбайт", available_memory / ( 1024 * 1024));
    }
}