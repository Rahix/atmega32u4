import toml
import textwrap

def group_blocks(registers):
    blocks = []
    current_block = {}

    for reg in registers:
        reg["addr"] = int(reg["addr"], 16)

    registers = list(sorted(registers, key=lambda r: r["addr"]))
    base = registers[0]["addr"]

    for i, reg in enumerate(registers):
        # Check if this is the same block as before
        if registers[i-1]["addr"] + 1 != reg["addr"]:
            if i != 0:
                blocks.append(current_block)
            current_block = {"addr": reg["addr"], "regs": []}

        current_block["regs"].append(reg)

    blocks.append(current_block)

    return (base, blocks)

def peripheral(filename):
    data = toml.load(open(filename))

    base, blocks = group_blocks(data["reg"])

    info = data["info"]
    print(f"""\
    <peripheral>
      <name>{info["name"]}</name>
      <description>{info["desc"]}</description>
      <baseAddress>{hex(base)}</baseAddress>
""")

    for block in blocks:
        print(f"""\
      <addressBlock>
        <offset>{hex(block["addr"]-base)}</offset>
        <size>{hex(len(block["regs"]))}</size>
        <usage>registers</usage>
      </addressBlock>""")

    print(f"""
      <registers>""")

    for block in blocks:
        for reg in block["regs"]:
            print(f"""\
        <register>
          <name>{reg["name"]}</name>
          <description>{reg["desc"]}</description>
          <addressOffset>{hex(reg["addr"]-base)}</addressOffset>""")

            if "cstm" in reg:
                print(textwrap.indent(reg["cstm"].strip(), "          "))

            if "fields" in reg:
                print("""
          <fields>""")

                for field in reg["fields"]:
                    custom = (
                        ""
                        if not "cstm" in field
                        else textwrap.indent(field["cstm"].strip(), "              ") + "\n"
                    )
                    print(f"""\
            <field>
              <name>{field["name"]}</name>
              <description>{field["desc"]}</description>
              <bitRange>[{field["rnge"]}]</bitRange>
{custom}            </field>""")

                print("""\
          </fields>""")

            print("""\
        </register>""")

    print(f"""\
      </registers>
    </peripheral>""")


if __name__ == "__main__":
    import sys
    peripheral(sys.argv[1])
