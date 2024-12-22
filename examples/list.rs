#[cfg(not(any(target_os = "windows", all(unix, not(target_os = "ios"), not(target_os = "android")))))]
fn main() {
    println!("This is currently only supported on Windows, Linux, and other Freedesktop.org compliant OSes");
}

#[cfg(any(target_os = "windows", all(unix, not(target_os = "ios"), not(target_os = "android"))))]
fn main() {
    use chrono::{DateTime, Local, NaiveDateTime, Utc};
    let trash_items = trash::os_limited::list().unwrap();

    let now = Local::now();
    let days = 42;
    let long_time_ago = now - chrono::Duration::days(days);
    let (old, new): (Vec<_>, Vec<_>) = trash_items.iter().partition(|item| {
        let naive_deletion_utc = NaiveDateTime::from_timestamp(item.time_deleted, 0);
        let deletion = DateTime::<Utc>::from_utc(naive_deletion_utc, Utc);
        deletion < long_time_ago
    });

    println!("There are {} items in your trash, older than {} days.", old.len(), days);
    println!("There are {} items in your trash, newer than {} days.", new.len(), days);
}
