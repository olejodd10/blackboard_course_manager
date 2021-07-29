use std::path::Path;

mod subject;
use subject::Subject;

fn main() {
    let statistikk = subject::WikiSubject::new(
        "TMA4245", 
        "2021v", 
        Path::new("./"), 
        ["https://www.math.ntnu.no/emner/TMA4245/2021v/skriftlige_ovinger/inn","-oppg-b.pdf"]
        .iter().map(|s| String::from(*s)).collect(),
    );

    statistikk.fetch_available_appointments().unwrap();
}
