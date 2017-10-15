import * as pbwrap from "google-protobuf/google/protobuf/wrappers_pb"
import { Message } from 'google-protobuf'

interface ProtoWrapperClass<T> {
    new (): ProtoWrapper<T>
}

interface ProtoWrapper<T> extends Message {
    getValue: () => T
    setValue: (valueObject: T) => void
}

interface OptionalSetter<T> {
    (
        setter: (protoValue: ProtoWrapper<T> | undefined) => void,
        valueObject: { value: T } | undefined
    ): void
}

function optionalFactory<T>(wrapper: ProtoWrapperClass<T>): OptionalSetter<T> {
    return (setter, valueField) => {
        if (valueField) {
            const protoValue = new wrapper()
            protoValue.setValue(valueField.value)
            setter(protoValue)
        }
    }
}

export const optionalStr = optionalFactory<string>(pbwrap.StringValue)
export const optionalI32 = optionalFactory<number>(pbwrap.Int32Value)
