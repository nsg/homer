# Homer Docs

## API

### Hue

You need to specify two environment variables:

    HUE_IP=127.0.0.1:8080 # The IP/DNS and possible port to the bridge.
    HUE_TOKEN=123...      # A token (username) already registered with the bridge.

#### Config

Dump the configuration from the bridge in JSON format.

    /api/hue/config/

Only dump part of the configuration, for example use `/api/hue/config/name` to return the bridges name.

    /api/hue/config/<str>

#### Lamps

Return all lamps registered with the bridge.

    /api/hue/lamps/

Only return the lamp with the specified id.

    /api/hue/lamps/<id>

Only return the lamp with the specified name. Note that this is an exact match.

    /api/hue/lamps/<name>
