#!/usr/bin/env python3
import json

def bit_fields(*, doc, one, onedoc, zero, zerodoc):
    print(f"""\
          <fields>""")
    for i in range(8):
        print(f"""\
            <field>
              <name>P{i}</name>
              <description>Pin {i} {doc}</description>
              <bitRange>[{i}:{i}]</bitRange>
              <enumeratedValues>
                <enumeratedValue>
                  <name>{zero}</name>
                  <description>{zerodoc}</description>
                  <value>0</value>
                </enumeratedValue>
                <enumeratedValue>
                  <name>{one}</name>
                  <description>{onedoc}</description>
                  <value>1</value>
                </enumeratedValue>
              </enumeratedValues>
            </field>""")
    print(f"""\
          </fields>""")

def main():
    print("""\
<?xml version="1.0" encoding="utf-8"?>

<device schemaVersion="1.3" xmlns:xs="http://www.w3.org/2001/XMLSchema-instance" xs:noNamespaceSchemaLocation="CMSIS-SVD.xsd">
  <vendor>Arduino LLC</vendor>
  <name>Arduino_Leonardo</name>
  <version>0.1</version>
  <description>Arduino Leonardo</description>

  <cpu>
    <name>Atmega32u4</name>
    <revision>r0p0</revision>
    <endian>little</endian>
    <mpuPresent>false</mpuPresent>
    <fpuPresent>true</fpuPresent>
    <nvicPrioBits>8</nvicPrioBits>
    <vendorSystickConfig>false</vendorSystickConfig>
  </cpu>

  <addressUnitBits>8</addressUnitBits>
  <size>8</size>
  <access>read-write</access>
  <resetValue>0</resetValue>
  <resetMask>0xff</resetMask>

  <peripherals>""")

    data = json.load(open("leonardo.json"))

    for peripheral in data:
        ty = peripheral["type"]
        name = peripheral["name"]
        doc_name = peripheral["doc_name"]
        base_addr = peripheral["base"]
        if ty == "port":
            print(f"""\
    <peripheral>
      <name>{name}</name>
      <baseAddress>{base_addr}</baseAddress>

      <addressBlock>
        <offset>0</offset>
        <size>0x03</size>
        <usage>registers</usage>
      </addressBlock>

      <registers>
        <register>
          <name>PIN</name>
          <description>{doc_name} Input</description>
          <access>read-only</access>
          <addressOffset>0x00</addressOffset>""")
            bit_fields(
                doc="Input",
                one="HIGH",
                onedoc="Pin is high",
                zero="LOW",
                zerodoc="Pin is low",
            )
            print(f"""\
        </register>

        <register>
          <name>DDR</name>
          <description>{doc_name} Direction</description>
          <addressOffset>0x01</addressOffset>""")
            bit_fields(
                doc="Direction",
                one="OUTPUT",
                onedoc="Pin is configured as an output",
                zero="INPUT",
                zerodoc="Pin is configured as an input",
            )
            print(f"""\
        </register>

        <register>
          <name>PORT</name>
          <description>{doc_name} Output</description>
          <access>write-only</access>
          <addressOffset>0x02</addressOffset>""")
            bit_fields(
                doc="Output",
                one="HIGH",
                onedoc="Pin is high",
                zero="LOW",
                zerodoc="Pin is low",
            )
            print(f"""\
        </register>
      </registers>
    </peripheral>""")

    print("""\
  </peripherals>
</device>""")


if __name__ == "__main__":
    main()
