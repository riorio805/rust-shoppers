# rusty business


### reflection 1
The code works 👍👍

Dalam fungsi `main()`:\
`TcpListener` mendengar di address **127.0.0.1** (localhost) dengan port **7878**, dan menerima request yang masuk untuk diproses dalam fungsi `handle_connection()`. For loop yang digunakan untuk mengiterasi `TcpListener` tidak akan pernah keluar karena `listener.incoming() -> Incoming` akan terus menerima request.

Dalam fungsi `handle_connection()`:\
`buf_reader` merupakan sebuah buffer `BufReader` yang terkonek lewat TCP, yang akan menerima request yang datang.
`http_request` merupakan array String yang terproses dari `buf_reader`, yang di print ke terminal.


### reflection 2
The code works 👍👍

HTML harus direturn sesuai protocol, dengan header `"HTTP/1.1 200 OK"`, lalu `Content-Length:` dengan length htmlnya, baru content tersebut.
Juga digunakan modul `fs` untuk mengakses file html `hello.html` untuk di kirimkan kembali ke browser.
Untuk mengirim respons HTML dapat menggunakan `TCPStream.write_all` pada respons tersebut lalu `unwrap` untuk mengembalikan value `Ok` stream tersebut.

![commit 2 screen capture](/archiveme/m2_working.png)


### reflection 3
The code works 👍👍

Untuk membedakan antara endpoint yang valid dengan yang tidak pada request, diambil endpoint dari `http_request` dan mengubahnya ke sebuah string immutable (karena hanya butuh di-read), lalu menggunakan `match` untuk return kode status dan file HTML untuk digunakan dalam respons dari request tersebut.

```rs
fn handle_connection(mut stream: TcpStream) {
    // -- snip --
    
    let request_line = http_request[0].as_str();
    let (status_line, filename) =
    match request_line {
        "GET / HTTP/1.1"
            => ("HTTP/1.1 200 OK", "hello.html"),
        _ 
            => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // -- snip --
}
```

![Hello page screen capture](/archiveme/m3_ok.png)
![Not available page screen capture](/archiveme/m3_bad.png)


### Reflection 4
Setelah mengubah code tersebut, dijalankan endpoint `localhost:7878/sleep`, lalu `localhost:7878/`.
Yang terjadi adalah keduanya akan tetap loading terus, meskipun hanyna endpoint `/sleep` yang ditambah delay.
Hal ini terjadi karena server masih single-threaded, yang menyebabkan endpoint yang tidak ada delay tetap menunggu pada
satu thread yang memproses endpoint `/sleep` yang menunggu waktu lama.

![Sleep endpoint](/archiveme/m4_sleep.png)
![Alt text](/archiveme/m4_delayed.png)