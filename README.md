# neptun-calendar-filter

## Webserver endpoints
- `/`
- `/filter/{id}`
- `/inverse-filter/{id}`

## Required enviorment values
*(.env also supported)*
```bash
DOMAIN="neptun.domain.com"
IP="0.0.0.0"
PORT="9876"
```

## Running the server
```bash
cargo run
```

---

## Used Neptun URL format inside the code
```Rust
"https://{domain}/hallgato/api/Calendar/CalendarExportFileToSyncronization?id={id}.ics"
```

## License

`neptun-calendar-filter` is licensed under the [MIT License](LICENSE.txt). You're free to use, modify, and distribute the code as you see fit.

