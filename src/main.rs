use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    let num_bytes = nic.recv(&mut buf[..])?;
    eprintln!("read:{}, bytes:{:x}", num_bytes, buf[..num_bytes]);
    Ok(())
}
