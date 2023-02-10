use std::{io, net::TcpListener};

/// Asks the kernel for a free open port that is ready to use.
pub fn get_free_port() -> io::Result<u16> {
    match TcpListener::bind("127.0.0.1:0") {
        Ok(listener) => Ok(listener.local_addr().expect("get local addr fails").port()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;

    use crate::get_free_port;

    #[test]
    fn test_get_free_port() {
        let port1 = get_free_port().expect("get free port fails");
        assert_eq!(port1 > 1, true);
        assert_eq!(port1 < 65534, true);

        // Try to listen on the port
        let port2 = TcpListener::bind(format!("127.0.0.1:{}", port1))
            .expect("using free port listen falis")
            .local_addr()
            .unwrap()
            .port();

        assert_eq!(port1, port2);
    }
}
