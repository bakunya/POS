pub trait Scanner {
    fn scan(&self, barcode: &str);
}

pub trait Printer {
    fn print_receipt(&self, content: &str);
}

pub trait CashDrawer {
    fn open(&self);
}

pub struct MockHardware;

impl Scanner for MockHardware {
    fn scan(&self, barcode: &str) {
        println!("[MOCK SCANNER] Scanned: {}", barcode);
    }
}

impl Printer for MockHardware {
    fn print_receipt(&self, content: &str) {
        println!("[MOCK PRINTER] Printing Receipt:\n{}", content);
    }
}

impl CashDrawer for MockHardware {
    fn open(&self) {
        println!("[MOCK DRAWER] Drawer Opened");
    }
}
