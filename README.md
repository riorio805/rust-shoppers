# rusty business


### reflection 1
The code works 👍👍

`buf_reader` merupakan sebuah buffer `BufReader` yang terkonek lewat TCP, yang akan menerima request yang datang.
`http_request` merupakan array String yang terproses dari `buf_reader`, yang di print ke terminal.


### reflection 2
The code works 👍👍

HTML harus direturn sesuai protocol, dengan header `"HTTP/1.1 200 OK"`, lalu `Content-Length:` dengan length htmlnya, baru content tersebut.
Juga digunakan modul `fs` untuk mengakses file html `hello.html` untuk di kirimkan kembali ke browser.
Untuk mengirim respons HTML dapat menggunakan `TCPStream.write_all` pada respons tersebut lalu `unwrap` untuk mengembalikan value `Ok` stream tersebut.

![commit 2 screen capture](archiveme\commit2.png)
