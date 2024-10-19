import protobufjs from 'protobufjs';
import SpaceProto from './spaceproto.json';

const root = protobufjs.Root.fromJSON(SpaceProto);
const WebsocketMessage = root.lookupType('WebsocketMessage');

export {
    WebsocketMessage,
}