pub fn date() -> String {
    //! returns the formatted string for the Date in yaml format
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S %z\n");

    eprintln!("{now}");

    format!("date:\t{now}",)
}
