# Examples

Below are a list of examples included with this library. They can be run by using one of the following:
- `cargo run --example EXAMPLE_NAME --features EXAMPLE_NAME`
- `cargo build --example EXAMPLE_NAME --release --features EXAMPLE_NAME`
- `docker build -t EXAMPLE_NAME --build-arg example=EXAMPLE_NAME .` (While in the root directory)

## Client

This library includes an example client for communicating with broadlink devices.
The source for it is [here](rbroadlink-cli.rs) and its usage is shown below:

```sh
rbroadlink 0.2.1
Nicholas Cioli <nicholascioli@gmail.com>, Wyatt Lindquist <git.wquist@gmail.com>
A library to control broadlink smart devices.

USAGE:
    rbroadlink-cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    blast      Blasts an IR / RF code to the world
    connect    Connect a broadlink device to the network. Only tested on the RM3 Mini and the
               RM4 Pro
    help       Print this message or the help of the given subcommand(s)
    info       Get information about a broadlink device
    learn      Learn a code from a broadlink device on the network
    list       Lists available broadlink devices on the network
```

An example of using the cli to learn an IR code for a device at 10.8.0.1 is shown below:

```sh
cargo run --example rbroadlink-cli --features rbroadlink-cli -- learn 10.8.0.1 ir
```

## MQTT Bridge

This library includes an example MQTT bridge for allowing home automation systems to interact
with broadlink devices. This requires the use of an MQTT broker (which support MQTT version 3.1.1)
, such as [mosquitto](https://mosquitto.org/).

The source can be found [here](mqtt-broadlink.rs) and its usage is shown below:

```sh
rbroadlink 0.2.1
Nicholas Cioli <nicholascioli@gmail.com>, Wyatt Lindquist <git.wquist@gmail.com>
A library to control broadlink smart devices.

USAGE:
    mqtt-broadlink [OPTIONS] <MQTT_BROKER>

ARGS:
    <MQTT_BROKER>    The MQTT broker used for publishing / subscribing to topics

OPTIONS:
    -a, --auto-discover
            Enable automatically discovering devices

        --auto-connect
            Automatically connect to the broker

    -c, --client <CLIENT>
            Add a client to track. Can be repeated

    -h, --help
            Print help information

        --keep-alive <KEEP_ALIVE>
            The keepalive interval, in seconds [default: 30]

        --local-ip <LOCAL_IP>
            Specify the local IP of this machine, if multiple interfaces are available

        --mqtt-id <MQTT_ID>
            The MQTT ID to use for this client, if needed [default: mqtt-broadlink]

        --operation-timeout <OPERATION_TIMEOUT>
            The operation timeout, in seconds [default: 20]

    -p, --password <PASSWORD>
            The MQTT password, if needed

    -u, --username <USERNAME>
            The MQTT username, if needed

    -V, --version
            Print version information
```

An example of using the bridge with a statically defined client at 10.8.0.1 and an insecure MQTT broker at
mqtt://1.2.3.4:1883 is shown below:

```sh
cargo run --example mqtt-broadlink --features mqtt-broadlink -- -c 10.8.0.1 mqtt://1.2.3.4:1883
```

## HVAC client

This library includes an example HVAC/Air Conditioner client to show how to control supported devices.

The source can be found [here](hvac-cli.rs) and its usage is shown below:

```sh
USAGE:
    hvac-cli MODE

MODE:
    info      show air conditioner state
    on        power ON air conditioner
    off       power OFF air conditioner
    toggle    toggle power state
```

Note: by default this client is autodiscovering all devices and it is trying to issue the command on all discovered devices.

An example of using this client to obtain information of the current state is show below:

```sh
cargo run --example hvac-cli -- info
```
