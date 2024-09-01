# Weathema
## Description
A simple weather app that displays the current weather of a city. This is a playground fortesting out the [Anathema TUI](https://togglebyte/anathema) library.

Use the tab key to cycle to the top left enter location widget (it will highlight in green whenit has focus). Enter the city name and press enter to fetch the weather data.

### Widgets
- Location widget: A simple text input widget that allows the user to enter the city name.
- Weather widget: A widget that displays the current weather of the city.
- Spinner widget: A simple spinner widget that spins when the app is fetching data (bottom left).
- Error widget: A widget that displays an error message when the app fails to fetch data.

## Usage
```bash
cargo run <city>
```

Ctrl-C to exit the app.

![usage.gif](docs/usage.gif)