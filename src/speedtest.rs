struct SpeedTestConfig;


pub fn speedtest() {
    println!("Starting Speed Test");
    let config = get_config();
}

fn get_config() -> SpeedTestConfig {
    SpeedTestConfig
}

fn distance() {

}

#[cfg(test)]
mod tests {
    extern crate xml;
    use std::io::{File, BufferedReader};
    use self::xml::reader::EventReader;
    use self::xml::reader::events::*;

    #[test]
    fn read_xml() {
        let file = File::open(&Path::new("data/speedtest-config.php.xml")).unwrap();
        let reader = BufferedReader::new(file);

        let mut parser = EventReader::new(reader);
        for e in parser.events() {
            println!("{:?}", e);
            // match e {
                // XmlEvent::StartElement {name, attributes: attr, namespace: _ } => {
                // }
                // _ => {}
            // }
        }
    }

    #[test]
    fn test_distance() {

    }
}
