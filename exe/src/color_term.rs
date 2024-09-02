/// https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences
pub trait VTerm {
    fn fg_green(self) -> String;

    fn bg_red(self) -> String;
}

impl VTerm for &str {
    fn fg_green(self) -> String {
        format!("\u{001B}[32m{self}\u{001B}[0m")
    }

    fn bg_red(self) -> String {
        format!("\u{001B}[41m{self}\u{001B}[0m")
    }
}
