fn main() {
    use modbus::{Client};
    use modbus::tcp;

    let cfg = tcp::Config::default();

    // Simply connect to modbus master at address 127.0.0.1 with default port 502
    let mut client = tcp::Transport::new_with_cfg("127.0.0.1", cfg).unwrap();
    let res = client.read_input_registers(101, 2).unwrap();

    match client.close() {
        Err(err) => println!("error on close : {}", err),
        _ => {}
    }

    for (idx, value) in res.iter().enumerate() {
        // We know that these values ​​are temperatures (ip thermometer with PoE)
        // So we format with "°C"
        println!("value {} : {:.2}°C", idx+1, (*value as f32)/10.0)
    }
}
