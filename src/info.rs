use std::thread::sleep;
use sysinfo::{CpuRefreshKind, RefreshKind, System, MINIMUM_CPU_UPDATE_INTERVAL};


fn main() {
    let mut s = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );
    let memoire: Vec<String> = utilisation_memoire(&mut s);

    println!("=== ÉTAT DU SYSTÈME ===");
    println!("Processeurs logiques: {}", nombre_processeurs(&mut s));
    println!("\nUtilisation des coeurs: {}", utilisation_coeurs(&mut s));
    println!("\nMémoire: \n Utilisée: {} Go\n Totale: {} Go", memoire[0], memoire[1]);

}

fn nombre_processeurs(sys: &mut System) -> String{
    sleep(MINIMUM_CPU_UPDATE_INTERVAL); 
    sys.refresh_cpu_usage();
    format!("{}", sys.cpus().iter().count())
}

fn utilisation_coeurs(sys: &mut System) -> String{
    let mut resultat = String::new();
    let mut x = 0;

    sleep(MINIMUM_CPU_UPDATE_INTERVAL); 
    sys.refresh_cpu_usage();
    for cpu in sys.cpus() {
        resultat.push_str(format!("\n Coeur {}: {}%", x, format!("{:.1}", cpu.cpu_usage()) ).as_str());
        x += 1;
    }

   resultat
}

fn utilisation_memoire(sys: &mut System) -> Vec<String>{
    sleep(MINIMUM_CPU_UPDATE_INTERVAL); 
    sys.refresh_memory();
    vec![
        format!("{:.1}",(sys.used_memory() as f64 / 1_000_000_000.0)),
        format!("{:.1}",(sys.total_memory() as f64 / 1_000_000_000.0)),
    ]
}