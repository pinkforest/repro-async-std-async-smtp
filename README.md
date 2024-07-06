# repro-async-std-async-smtp

```
 cargo build
   Compiling repro-async-std v0.1.0 (/home/foobar/code/repro/repro-async-std)
error[E0277]: the trait bound `async_std::net::TcpStream: async_std::io::BufRead` is not satisfied
   --> src/main.rs:9:52
    |
9   |     let mut transport = SmtpTransport::new(client, stream).await.unwrap();
    |                         ------------------         ^^^^^^ the trait `async_std::io::BufRead` is not implemented for `async_std::net::TcpStream`
    |                         |
    |                         required by a bound introduced by this call
    |
    = help: the following other types implement trait `async_std::io::BufRead`:
              Box<T>
              futures_lite::io::Empty
              futures_lite::io::Empty
              futures_lite::io::BufReader<R>
              futures_lite::io::BufReader<R>
              futures_lite::io::Cursor<T>
              async_std::io::Empty
              futures_lite::io::Cursor<T>
            and 20 others
note: required by a bound in `SmtpTransport::<S>::new`
   --> /home/foobar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/async-smtp-0.9.1/src/smtp_client.rs:103:9
    |
103 | impl<S: BufRead + Write + Unpin> SmtpTransport<S> {
    |         ^^^^^^^ required by this bound in `SmtpTransport::<S>::new`
104 |     /// Creates a new SMTP transport and connects.
105 |     pub async fn new(builder: SmtpClient, stream: S) -> Result<Self, Error> {
    |                  --- required by a bound in this associated function

error[E0277]: the trait bound `async_std::net::TcpStream: async_std::io::BufRead` is not satisfied
   --> src/main.rs:9:25
    |
9   |     let mut transport = SmtpTransport::new(client, stream).await.unwrap();
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `async_std::io::BufRead` is not implemented for `async_std::net::TcpStream`
    |
    = help: the following other types implement trait `async_std::io::BufRead`:
              Box<T>
              futures_lite::io::Empty
              futures_lite::io::Empty
              futures_lite::io::BufReader<R>
              futures_lite::io::BufReader<R>
              futures_lite::io::Cursor<T>
              async_std::io::Empty
              futures_lite::io::Cursor<T>
            and 20 others
note: required by a bound in `SmtpTransport<S>`
   --> /home/foobar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/async-smtp-0.9.1/src/smtp_client.rs:103:9
    |
103 | impl<S: BufRead + Write + Unpin> SmtpTransport<S> {
    |         ^^^^^^^ required by this bound in `SmtpTransport<S>`

error[E0277]: the trait bound `async_std::net::TcpStream: async_std::io::BufRead` is not satisfied
   --> src/main.rs:9:60
    |
9   |     let mut transport = SmtpTransport::new(client, stream).await.unwrap();
    |                                                            ^^^^^ the trait `async_std::io::BufRead` is not implemented for `async_std::net::TcpStream`
    |
    = help: the following other types implement trait `async_std::io::BufRead`:
              Box<T>
              futures_lite::io::Empty
              futures_lite::io::Empty
              futures_lite::io::BufReader<R>
              futures_lite::io::BufReader<R>
              futures_lite::io::Cursor<T>
              async_std::io::Empty
              futures_lite::io::Cursor<T>
            and 20 others
note: required by a bound in `SmtpTransport<S>`
   --> /home/foobar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/async-smtp-0.9.1/src/smtp_client.rs:103:9
    |
103 | impl<S: BufRead + Write + Unpin> SmtpTransport<S> {
    |         ^^^^^^^ required by this bound in `SmtpTransport<S>`

error[E0277]: the trait bound `async_std::net::TcpStream: async_std::io::BufRead` is not satisfied
  --> src/main.rs:9:25
   |
9  |     let mut transport = SmtpTransport::new(client, stream).await.unwrap();
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `async_std::io::BufRead` is not implemented for `async_std::net::TcpStream`
   |
   = help: the following other types implement trait `async_std::io::BufRead`:
             Box<T>
             futures_lite::io::Empty
             futures_lite::io::Empty
             futures_lite::io::BufReader<R>
             futures_lite::io::BufReader<R>
             futures_lite::io::Cursor<T>
             async_std::io::Empty
             futures_lite::io::Cursor<T>
           and 20 others
note: required by a bound in `SmtpTransport`
  --> /home/foobar/.cargo/registry/src/index.crates.io-6f17d22bba15001f/async-smtp-0.9.1/src/smtp_client.rs:94:29
   |
94 | pub struct SmtpTransport<S: BufRead + Write + Unpin> {
   |                             ^^^^^^^ required by this bound in `SmtpTransport`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `repro-async-std` (bin "repro-async-std") due to 4 previous errors
```