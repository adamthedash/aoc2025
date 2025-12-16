use std::io::stdin;

pub fn parse_input() -> (Vec<String>, Vec<(usize, Vec<usize>)>) {
    let mut devices = vec![];
    let connections = stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let (device, outputs) = line.split_once(": ").expect("Failed to split line");

            let device = if let Some(i) = devices.iter().position(|d| d == device) {
                i
            } else {
                devices.push(device.to_string());
                devices.len() - 1
            };

            let outputs = outputs
                .split(" ")
                .map(|device| {
                    if let Some(i) = devices.iter().position(|d| d == device) {
                        i
                    } else {
                        devices.push(device.to_string());
                        devices.len() - 1
                    }
                })
                .collect();

            (device, outputs)
        })
        .collect::<Vec<_>>();

    (devices, connections)
}
