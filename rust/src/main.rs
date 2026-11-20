use crate::api::grade::Grade;
use crate::api::{DrillMemory, local_now};

mod api;

fn main() {
    let mut drill =
        DrillMemory::new(api::DrillLanguage::RUST, "If Statement".into());

    drill.review(Grade::Good);
    let now = local_now();
    println!("{}", drill.retrievability(now));
    println!("{}", drill.retrievability(now + 86400));
    println!("{}", drill.retrievability(now + 86400 * 7));
    println!("{}", drill.due_date_str());
}
