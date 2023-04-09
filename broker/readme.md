# Connect packet processing

Following sections extracted from spec

- After a Network Connection is established by a Client to a Server, the first packet sent from the Client to the Server MUST be a CONNECT packet [MQTT-3.1.0-1].
- The Server MUST process a second CONNECT packet sent from a Client as a Protocol Error and close the Network Connection [MQTT-3.1.0-2].
- The protocol name MUST be the UTF-8 String "MQTT". If the Server does not want to accept the CONNECT, and wishes to reveal that it is an MQTT Server it MAY send a CONNACK packet with Reason Code of 0x84 (Unsupported Protocol Version), and then it MUST close the Network Connection [MQTT-3.1.2-1]


## Ideas

- Could probably have one async function which takes ToSocketAddrs and creates two more task inside. first task will be 
serialiser and second task will be deserialiser.
- 