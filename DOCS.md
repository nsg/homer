# Homer Docs

## API

### Hue

You need to specify two environment variables:

    HUE_IP=127.0.0.1:8080 # The IP/DNS and possible port to the bridge.
    HUE_TOKEN=123...      # A token (username) already registered with the bridge.

#### Config

    /api/hue/config/

Dump the configuration from the bridge in JSON format.

    /api/hue/config/<str>

Only dump part of the configuration, for example use `/api/hue/config/name` to return the bridges name.
