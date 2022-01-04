#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(unix, not(feature = "server"), not(target_os = "macos")))]
fn main() {
    println!("this is a xdg only feature")
}

use std::{error::Error, thread::sleep, time::Duration};
use zbus::{dbus_interface, fdo, Connection, ObjectServer};
#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
#[async_std::main]
async fn main() {
    struct Greeter {
        count: u64,
    }

    #[dbus_interface(name = "org.zbus.MyGreeter1")]
    impl Greeter {
        // Can be `async` as well.
        fn say_hello(&mut self, name: &str) -> String {
            self.count += 1;
            format!("Hello {}! I have been called: {}", name, self.count)
        }
    }

    async fn main() -> Result<(), Box<dyn Error>> {
        let mut greeter = Greeter { count: 0 };
        let connection = Connection::new_for_address("/org/zbus/MyGreeter")?;
        connection.request_name("org.zbus.MyGreeter").await?;

        // Do other things or go to sleep.
        sleep(Duration::from_secs(60));

        Ok(())
    }
}
