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

Only return the lamp with the specified id or name.

    GET /api/hue/lights/<id|name>

Inspect the software version of the lamps

    GET /api/hue/lights/version

Turn a lamp on or off

    PUT /api/hue/lights/<id|name>/on/{true,false}

Set lamp brightness

    PUT /api/hue/lights/<id|name>/brightness/<0-254>

Set lamp color from hex rgb (like ff0000)

    PUT /api/hue/lights/<id|name>/color/<hex>

Blink a lamp once, or for 10 seconds

    PUT /api/hue/lights/<id|name>/alert/<mode>

    mode = 1       Blink the lamp once
    mode = 10      Blick for 10 seconds
