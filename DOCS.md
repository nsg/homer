# Homer Docs

## API

### Hue

You need to specify two environment variables:

    HUE_IP=127.0.0.1:8080 # The IP/DNS and possible port to the bridge.
    HUE_TOKEN=123...      # A token (username) already registered with the bridge.
    HUE_PORT=80           # Specify the port (defaults to 80)

#### Config

Dump the configuration from the bridge in JSON format.

    GET /api/hue/config/

Only dump part of the configuration, for example use `/api/hue/config/name` to return the bridges name.

    GET /api/hue/config/<str>

#### Lamps

Return all lamps registered with the bridge.

    GET /api/hue/lights/

Only return the lamp with the specified id.

    GET /api/hue/lights/<id>

Only return the lamp with the specified name. Note that this is an exact match.

    GET /api/hue/lights/<name>

Turn a lamp on or off

    PUT /api/hue/lights/<id>/on/{true,false}

Set lamp brightness

    PUT /api/hue/lights/<id>/brightness/<0-254>
