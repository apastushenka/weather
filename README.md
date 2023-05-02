# weather
A simple weather CLI application

### Building
```
git clone https://github.com/apastushenka/weather.git
cd weather
cargo build --release
./target/release/weather --help
```

### Usage
Get all available weather providers:
```
weather providers
```

Configure weather provider:
```
weather configure <PROVIDER>
```

Set default provider:
```
weather default <PROVIDER>
```

Get weather using default provider:
```
weather get <ADDRESS> [DATE]
```
