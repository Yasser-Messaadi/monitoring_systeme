use sysinfo::{CpuRefreshKind, RefreshKind, System};


fn main() {
    let s = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );

    println!("=== ÉTAT DU SYSTÈME ===");
    println!("{}", nombre_processeurs(s));
    //println!("{}", utilisation_coeurs(s))

}

fn nombre_processeurs(mut sys: System) -> String{
    sys.refresh_cpu_usage();
    format!("Processeurs logiques : {}", sys.cpus().iter().count())
}

fn utilisation_coeurs(mut sys: System) -> String{
    let mut resultat = String::new();
    let mut liste_usage_cpus: Vec<f32> = Vec::new();
    let mut x = 0;
    sys.refresh_cpu_usage();
    for cpu in sys.cpus() {
        liste_usage_cpus.push(cpu.cpu_usage());
        resultat.push_str(format!("\n Coeur {}: {}%", x, cpu.cpu_usage()).as_str());
        x += 1;
    }

   resultat
}
