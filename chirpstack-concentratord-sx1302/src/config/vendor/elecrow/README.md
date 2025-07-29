# Elecrow HAT Configuration

This configuration adds support for the Elecrow LoRaWAN HAT to chirpstack-concentratord-sx1302.

## Model Name

To use the Elecrow HAT, set the gateway model in your configuration to:
```
model = "elecrow_sx1302_lorawan_gateway_hat"
```

## Pin Configuration

The Elecrow HAT differs from the Waveshare HAT in the sx1302_reset_pin:
- Waveshare HAT: uses GPIO pin 23
- Elecrow HAT: uses GPIO pin 17

All other configuration parameters are identical to the Waveshare HAT.

## Usage Example

```toml
[gateway]
model = "elecrow_sx1302_lorawan_gateway_hat"
region = "EU868"
# ... other configuration options
```