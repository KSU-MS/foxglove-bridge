import cantools
from cantools.database import conversion, Message
import sys
import os


def main():
    if len(sys.argv) > 1:
        path_to_dbc = sys.argv[1]
    else:
        path_to_dbc = os.environ.get("DBC_PATH")

    if path_to_dbc is None:
        print("No DBC passed and no DBC_PATH var set")
        exit()

    db = cantools.database.load_file(path_to_dbc)

    with open(f"{os.path.basename(path_to_dbc)}.fbs", "w+") as fbs_file:
        fbs_file.write("namespace dbc;\n\n")

        for msg in db.messages:
            fbs_file = append_proto_message_from_CAN_message(fbs_file, msg)


def create_field_name(name: str) -> str:
    replaced_text = name.replace(" ", "_")
    replaced_text = replaced_text.replace("(", "")
    replaced_text = replaced_text.replace(")", "")
    return replaced_text


def append_proto_message_from_CAN_message(file, can_msg: Message):
    # Start the table
    file.write("table " + can_msg.name.lower() + " {\n")

    for sig in can_msg.signals:
        # if is_float + edge cases
        if (
            sig.is_float
            or ((sig.scale is not None) and (sig.scale != 1.0))
            or (
                type(sig.conversion)
                is not type(conversion.IdentityConversion(is_float=False))
                and not type(
                    conversion.NamedSignalConversion(
                        choices={}, scale=0, offset=0, is_float=False
                    )
                )
            )
        ):
            line = "   " + create_field_name(sig.name) + ":float;"

        # Enums with named values
        elif sig.choices is not None:
            line = "   " + create_field_name(sig.name) + ":string;"

        # True
        elif sig.length == 1:
            line = "   " + create_field_name(sig.name) + ":bool;"

        # The ints
        elif sig.length > 1 and sig.length <= 32:
            line = "   " + create_field_name(sig.name) + ":int32;"

        elif sig.length >= 32 and not sig.is_signed:
            line = "   " + create_field_name(sig.name) + ":uint64;"

        else:
            line = "   " + create_field_name(sig.name) + ":int64;"
        file.write(line + "\n")
    file.write("}\n\n")
    return file


if __name__ == "__main__":
    main()
