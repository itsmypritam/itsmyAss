use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // Bind UDP socket (use 5353 instead of 53 to avoid root permission)
    let socket = UdpSocket::bind("0.0.0.0:5353")?;
    println!("DNS server running on port 5353...");

    let mut buf = [0u8; 512];

    loop {
        let (size, src) = socket.recv_from(&mut buf)?;
        println!("Received {} bytes from {}", size, src);

        let request = &buf[..size];

        // Create response buffer
        let mut response = Vec::new();

        // Copy transaction ID
        response.extend_from_slice(&request[0..2]);

        // Flags: standard query response, no error
        response.extend_from_slice(&[0x81, 0x80]);

        // Questions: 1
        response.extend_from_slice(&[0x00, 0x01]);

        // Answer RRs: 1
        response.extend_from_slice(&[0x00, 0x01]);

        // Authority RRs: 0
        response.extend_from_slice(&[0x00, 0x00]);

        // Additional RRs: 0
        response.extend_from_slice(&[0x00, 0x00]);

        // Copy original question
        let mut i = 12;
        while request[i] != 0 {
            i += 1;
        }
        i += 5; // skip null byte + QTYPE (2) + QCLASS (2)

        response.extend_from_slice(&request[12..i]);

        // Answer section
        response.extend_from_slice(&[
            0xc0, 0x0c, // pointer to domain name
            0x00, 0x01, // TYPE A
            0x00, 0x01, // CLASS IN
            0x00, 0x00, 0x00, 0x3c, // TTL = 60 seconds
            0x00, 0x04, // RDLENGTH = 4 bytes
            127, 0, 0, 1, // IP = 127.0.0.1
        ]);

        socket.send_to(&response, src)?;
        println!("Sent response to {}", src);
    }
}
