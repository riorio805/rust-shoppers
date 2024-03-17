# rusty business

The grust never end, and the crumb don't stop.

## Reflection 1
The code works 👍👍

Dalam fungsi `main()`:\
`TcpListener` mendengar di address **127.0.0.1** (localhost) dengan port **7878**, dan menerima request yang masuk untuk diproses dalam fungsi `handle_connection()`.
For loop yang digunakan untuk mengiterasi `TcpListener` tidak akan pernah keluar karena `listener.incoming() -> Incoming` akan terus menerima request.

Dalam fungsi `handle_connection()`:\
`buf_reader` merupakan sebuah buffer `BufReader` yang terkonek lewat TCP, yang akan menerima request yang datang.
`http_request` merupakan array String yang terproses dari `buf_reader`, yang di print ke terminal.


## Reflection 2
The code works 👍👍

Untuk mengembalikan respons HTML, harus dibuat sesuai protocol.
Templatenya adalah sebuah header (seperti `"HTTP/1.1 200 OK"` untuk menunjukkan OK), lalu `Content-Length:` dengan length isi contentnya dalam jumlah bytes, baru content tersebut.
Content tersebut digunakan modul `fs` untuk mengakses file html `hello.html` untuk di kirimkan kembali ke browser.
Untuk mengirim respons HTML dapat menggunakan `TCPStream.write_all` pada respons tersebut lalu `unwrap` untuk mengembalikan value `Ok` stream tersebut.

![commit 2 screen capture](/archiveme/m2_working.png)


## Reflection 3
The code works 👍👍

Untuk membedakan antara endpoint yang valid dengan yang tidak pada request, diambil endpoint dari `http_request` dan mengubahnya ke sebuah string immutable (karena hanya butuh di-read).
String tersebut diproses menggunakan `match` untuk return kode status dan file HTML untuk digunakan dalam respons dari request tersebut.
Kita membuat 1 endpoint yang valid, beserta endpoint default ketika endpoint yang diminta tidak ada.
Endpoint yang valid (`/`) mengirim kode 200 dengan html di file `hello.html`.
Endpoint yang tidak valid (contohnya `/bad`) mengirim kode 404 dengan html di file `404.html`.

```rust
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


## Reflection 4
Kode tersebut diubah untuk menambah endpoint `/sleep`, yang mengirim content yang sama dengan endpoint `/` tetapi dengan delay 15 detik untuk simulasi delay pada server.
Setelah mengubah kode tersebut, dijalankan endpoint `localhost:7878/sleep`, lalu `localhost:7878/`.
Yang terjadi adalah keduanya akan tetap loading terus, meskipun hanyna endpoint `/sleep` yang ditambah delay.
Hal ini terjadi karena server masih single-threaded, yang menyebabkan endpoint yang tidak ada delay tetap menunggu pada satu thread yang memproses endpoint `/sleep` yang menunggu waktu lama.

![Sleep endpoint](/archiveme/m4_sleep.png)
![Alt text](/archiveme/m4_delayed.png)


## Reflection 5
The code works 👍👍

Untuk mengubah program menjadi multi-threaded, dapat digunakan method `thread::spawn` untuk membuat thread baru yang menjalankan fungsi yang diberikan (yaitu `handle_connection()`) setiap kali mendapat sebuah request.
Namun, ini bukan solusi yang bagus karena bisa saja seseorang dapat mengirim banyak request sekaligus dan mengakibatkan server menjadi overloaded dan terjadi crash.
Solusi yang tepat adalah kita hanya membuat beberapa thread saja, dan mengirim fungsi tersebut untuk dijalankan di sebuah thread jika ada.

Untuk implementasinya, dibuat class baru yaitu `ThreadPool` dan `Worker`, dan .
Class `Worker` menyimpan sebuah thread untuk menjalankan sebuah fungsi saat diberikan. 
Class `ThreadPool` untuk menyimpan `Worker` tersebut dan memilih worker yang bebas untuk diberikan fungsi yang diterima untuk dijalankan.
Fungsi tersebut dikirim sebagai tipe `Box<dyn FnOnce() + Send + 'static>`. Jelasnya adalah sebagai berikut:
- `dyn FnOnce() + Send + 'static` => Sebuah *trait object* untuk sebuah tipe objek. Trait tersebut digabungkan menggunakan `+`.
- `FnOnce()` => Trait ini menentukan bahwa objek tersebut dapat dipanggil (seperti fungsi), tetapi hanya sekali saja.
- `Send` => Trait ini menentukan bahwa objek ini dapat dikirim antar-thread.
- `'static` => Sebuah *lifetime bound* yang menentukan bahwa objek tersebut tidak akan menjadi invalid selama belum di-drop. Objek yang dapat memenuhi ini adalah variabel yang memiliki objek secara langsung  (bukan referensi). 
- `Box<...>` => Kita membungkus objek tersebut dengan sebuah `Box` untuk memastikan bahwa kita mengirim data dengan panjang yang sama, yaitu referensi ke fungsi tersebut.

Untuk komunikasi antara parent `ThreadPool` dengan child `Worker`, dibutuhkan modul `sync::mpsc` untuk meng-*handle* pengiriman data antara class tersebut.
Kita tidak bisa menggunakan objek `Receiver` dan `Sender` saja, karena pada implementasi di Rust, kita hanya dapat membuat 1 Receiver untuk banyak Sender, terbalik dengan yang kita mau.
Untuk mengatasi masalah tersebut, kita dapat menggunakan class `Mutex` dan `Arc`.
Class `Mutex` digunakan untuk memproteksi pembacaan Receiver secara bersamaan dengan implementasi sistem 'lock'.
Class `Arc` digunakan untuk memungkinkan _cloning_ objek untuk dikirim ke semua `Worker`.
Dengan menggabungkan kedua class untuk membuat objek `Arc<Mutex<Receiver>>>`, kita dapat cloning objek tersebut untuk dikirim ke semua `Worker`.

Di kelas `Worker`, dibuat 1 thread baru (juga disimpan) yang menunggu data dari Receiver untuk diproses.
Karena hanya 1 thread boleh mengakses Receiver tersebut, maka pertama `Mutex` tersebut di lock oleh sebuah thread.
Saat Mutex di-lock, jika thread lain memanggil lock pada mutex tersebut, maka thread lain tersebut akan menunggu sampai mutex di-unlock oleh thread awal, lalu langsung me-lock Mutex tersebut lagi.
Setelah Mutex di-lock oleh sebuah thread, thread tersebut langsung menunggu receiver untuk mengirim fungsi tersebut.
Setelah mendapat fungsi yang dikirim, thread meng-assign fungsi tersebut ke variabel local, lalu keluar dari scope Mutex.
Mutex akan mendeteksi ini dan langsung meng-unlock dirinya secara otomatis.
Fungsi yang didapat oleh thread tersebut langsung dijalankan, setelah selesai thread menunggu receiver lagi.

Implementasi tersebut ditaruh di file baru `lib.rs` dan di import untuk digunakan di `main.rs`.
Pada server, kita menggunakan 4 thread.
Hasilnya adalah server dapat meng-handle beberapa request sekaligus, sehingga jika dilakukan simulasi di Milestone 4,
maka endpoint `/` akan selesai dengan cepat meskipun endpoint `/sleep` masih menunggu.



## Refelction Bonus??
Fungsi `build` di `ThreadPool` bekerja sama dengan fungsi `new`, tetapi fungsi tersebut tidak panic ketika input size kurang dari sama dengan nol. Fungsi tersebut akan mengembalikan error saja.

Awalnya, kita buat struct `PoolCreationError` yang menyimpan error message dalam bentuk static str.
Struct ini berperan sebagai tipe output error.
Fungsi `build` akan return value `Result<ThreadPool, PoolCreationError>`, yang berarti fungsi tersebut mengembalikan ThreadPool jika tidak ada error, dan mengembalikan `PoolCreationError` jika terdapat error.
Untuk mengembalikan nilai error, maka kita menggunakan `Err` untuk membungkus `PoolCreationError`.
Untuk mengembalikan nilai sukses, maka kita menggunakan `Ok` untuk membungkus `ThreadPool`.

```rust
// return Result dengan ThreadPool jika sukses, dan PoolCreationError jika error
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    // menggantikan panic
    if size <= 0 {
        // menggunakan Err untuk mengembalikan nilai error
        return Err(PoolCreationError { error_message: "aaa" })
    }
    // dari ini ...
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);
    for id in 0..size {
        workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    // ... sampai ini sama persis dengan fungsi new()
    // menggunakan Ok untuk menunjukkan nilai success / tidak error
    Ok(ThreadPool { workers, sender })
}
```