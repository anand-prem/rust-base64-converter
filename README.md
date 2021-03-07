# rust-base64-converter

Save input string from a JSON object in base64 encoded string of PDF or PNG to a file by converting to buffer.

## Usage

### Build the binary
```sh
cargo build --release
```

### Provide execution permission
```sh
chmod +x <path_to_binary>
```

### Execute the binary
```sh
./converter --file <path to json file> --path <path_to_json_object> --output <output_file_name.extension(PNG or PDF)>
```

Eg:
```sh
./converter --file ~/Downloads/resp.json --path data.parcels.0.label.base64 --output label.pdf
```
