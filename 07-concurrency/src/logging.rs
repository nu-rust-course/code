use std::fmt;

/// Dummy client object (might be a `TcpStream` in your program).
#[derive(Clone)]
pub struct Client;

/// Dummy log message.
pub struct LogMessage;

impl fmt::Display for LogMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "logged")
    }
}

/// Logging using a shared Arc<Mutex<File>>.
mod mutex_version {
    use std::fs::File;
    use std::io::Write;
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    use super::{LogMessage, Client};

    type SharedLog = Arc<Mutex<File>>;

    fn web_server(log_file: File) {
        let log_arc = Arc::new(Mutex::new(log_file));

        for client in std::iter::repeat(Client) {
            let log_arc_clone = log_arc.clone();
            thread::spawn(move || handle_client(client, log_arc_clone));
        }
    }

    fn handle_client(client: Client, log_file: SharedLog) {

        {
            let mut guard = log_file.lock().unwrap();
            let _ = writeln!(*guard, "{}", LogMessage);
        } // unlocks here

    }
}

/// Logging by sending messages to a logging thread over a channel.
mod channel_version {
    use std::fs::File;
    use std::thread;

    use super::{LogMessage, Client};

    /// This module shows how you might encapsulate the logging thread
    /// into a simple API.
    mod logger {
        use std::fs::File;
        use std::io::Write;
        use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
        use std::thread;

        use super::super::LogMessage;

        /// The logger object, which holds the transmit end of the
        /// channel for sending log messages to the logging thread.
        #[derive(Clone)]
        pub struct Logger(SyncSender<LogMessage>);

        impl Logger {
            pub fn new(mut log_file: File) -> Self {
                let (tx, rx) = sync_channel(0);

                thread::spawn(move || {
                    for message in rx {
                        let _ = writeln!(log_file, "{}", message);
                    }
                });

                Logger(tx)
            }

            pub fn log(&self, message: LogMessage) {
                let _ = self.0.send(message);
            }
        }
    }

    use logger::Logger;

    fn web_server(log_file: File) {
        let logger = Logger::new(log_file);

        for client in std::iter::repeat(Client) {
            let my_logger = logger.clone();
            thread::spawn(move || handle_client(client, my_logger));
        }
    }

    fn handle_client(client: Client, logger: Logger) {

        logger.log(LogMessage);

    }
}

/// This version of the logger automatically stops the logging thread
/// when the last `Logger` object is dropped.
mod fancy_logger {
    use std::fs::File;
    use std::io::Write;
    use std::sync::{Arc, mpsc::{sync_channel, Receiver, SyncSender}};
    use std::thread;

    use super::LogMessage;
    use std::thread::JoinHandle;

    #[derive(Clone)]
    pub struct Logger(Arc<InnerLogger>);

    /// Since `Logger` contains an `Arc<InnerLogger>`, cloning a
    /// `Logger` doesn't clone the `InnerLogger`, and we can define a
    /// destructor (`Drop` impl) for `InnerLogger` that stops the logging
    /// thread.
    struct InnerLogger {
        tx:   SyncSender<Option<LogMessage>>,
        join: Option<thread::JoinHandle<()>>,
    }

    impl Drop for InnerLogger {
        fn drop(&mut self) {
            // When `InnerLogger` is dropped, we send a message to the logging
            // thread requesting it to finish and then wait for it.
            let _ = self.tx.send(None);
            let _ = self.join.take().map(JoinHandle::join);
        }
    }

    impl Logger {
        pub fn new(log_file: File) -> Self {
            let (tx, rx) = sync_channel(0);
            let join = thread::spawn(move || logging_loop(log_file, rx));
            Logger(Arc::new(InnerLogger { tx, join: Some(join) }))
        }

        pub fn log(&self, message: LogMessage) {
            let _ = self.0.tx.send(Some(message));
        }
    }

    fn logging_loop(mut log_file: File, rx: Receiver<Option<LogMessage>>) {
        for message_option in rx {
            if let Some(message) = message_option {
                let _ = writeln!(log_file, "{}", message);
            } else {
                // We've been asked to finish.
                return;
            }
        }
    }
}
